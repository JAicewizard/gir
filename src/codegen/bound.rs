use crate::{
    analysis::{
        bounds::{Bound, BoundType},
        ref_mode::RefMode,
    },
    library::Nullable,
};

impl Bound {
    /// Returns the type parameter reference.
    /// Currently always returns the alias.
    pub(super) fn type_parameter_reference(&self) -> char {
        self.alias
    }

    /// Returns the type parameter reference, with [`BoundType::IsA`] wrapped
    /// in `ref_mode` and `nullable` as appropriate.
    pub(super) fn full_type_parameter_reference(
        &self,
        ref_mode: RefMode,
        nullable: Nullable,
        r#async: bool,
    ) -> String {
        let t = self.type_parameter_reference();
        let ref_str = ref_mode.for_rust_type();
        match self.bound_type {
            BoundType::IsA(_) if *nullable => {
                format!("Option<{}{}>", ref_str, t)
            }
            BoundType::IsA(_) => format!("{}{}", ref_str, t),
            BoundType::ToGlibPtr(_, Some(lifetime)) => {
                let mut lifetime_post = r#async
                    .then(|| "'static".to_string())
                    .or_else(|| Some(format!(" '{}", lifetime)))
                    .unwrap_or_default();
                if ref_str.is_empty() {
                    lifetime_post = "".to_string();
                }
                format!("{}{} {}", ref_str, lifetime_post, t)

                // format!("{}", /* ref_str, lifetime_post, */ t)
            }
            BoundType::NoWrapper | BoundType::ToGlibPtr(_, _) | BoundType::AsRef(_) => {
                t.to_string()
            }
        }
    }

    /// Returns the type parameter definition for this bound, usually
    /// of the form `T: SomeTrait` or `T: IsA<Foo>`.
    pub(super) fn type_parameter_definition(&self, r#async: bool) -> String {
        format!("{}: {}", self.alias, self.trait_bound(r#async))
    }

    /// Returns the trait bound, usually of the form `SomeTrait`
    /// or `IsA<Foo>`.
    pub(super) fn trait_bound(&self, r#async: bool) -> String {
        match self.bound_type {
            BoundType::NoWrapper => self.type_str.clone(),
            BoundType::IsA(lifetime) => {
                if r#async {
                    assert!(lifetime.is_none(), "Async overwrites lifetime");
                }
                let is_a = format!("IsA<{}>", self.type_str);
                let lifetime = r#async
                    .then(|| " + Clone + 'static".to_string())
                    .or_else(|| lifetime.map(|l| format!(" + '{}", l)))
                    .unwrap_or_default();

                format!("{}{}", is_a, lifetime)
            }
            BoundType::AsRef(Some(_ /*lifetime*/)) => panic!("AsRef cannot have a lifetime"),
            BoundType::AsRef(None) => format!("AsRef<{}>", self.type_str),
            BoundType::ToGlibPtr(mutable, Some(lifetime)) => {
                eprintln!("tStr: {}", self.type_str);
                let modif = if mutable { "mut" } else { "const" };
                let post = r#async
                    .then(|| " + Clone + 'static".to_string())
                    .or_else(|| Some(format!(" + '{}", lifetime)))
                    .unwrap_or_default();
                let lifetime = r#async
                    .then(|| "'static".to_string())
                    .or_else(|| Some(format!("'{}", lifetime)))
                    .unwrap_or_default();

                format!(
                    "ToGlibPtr<{}, *{} libc::c_char> + ?Sized{}",
                    lifetime, modif, post
                )
            }
            BoundType::ToGlibPtr(_, None) => panic!("ToGlibPtr must have a lifetime"),
        }
    }
}
