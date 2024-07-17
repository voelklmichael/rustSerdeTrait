mod exact_resolver;
pub use exact_resolver::{ExactResolver, ExactResolverError};
pub use serde_trait_typeinfo::{TypeInfo, TypeInfoWithRustc};

#[derive(Debug, Default)]
pub struct RegistryInternal<T> {
    data: Vec<(TypeInfoWithRustc, T)>,
}

impl<T> RegistryInternal<T> {
    pub fn get<Resolver: RegistryConflictResolver>(
        &self,
        type_request: &TypeInfo,
        resolver: &Resolver,
    ) -> Result<Option<&T>, Resolver::Error> {
        let possibles = self
            .data
            .iter()
            .filter_map(|(type_info, value)| {
                resolver
                    .check(&type_info.type_info, type_request)
                    .then_some((type_info, value))
            })
            .collect::<Vec<_>>();
        if possibles.is_empty() {
            Ok(None)
        } else if possibles.len() == 1 {
            return Ok(Some(possibles[0].1));
        } else {
            resolver.resolve(type_request, possibles)
        }
    }
    pub fn register(&mut self, info: TypeInfoWithRustc, value: T) {
        self.data.push((info, value))
    }
}

pub trait RegistryConflictResolver {
    type Error;
    fn check(&self, type_request: &TypeInfo, type_info: &TypeInfo) -> bool;
    fn resolve<'a, T>(
        &self,
        type_request: &TypeInfo,
        possibles: Vec<(&'a TypeInfoWithRustc, &T)>,
    ) -> Result<Option<&'a T>, Self::Error>;
}
