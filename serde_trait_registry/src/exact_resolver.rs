use serde_trait_typeinfo::TypeInfoWithRustc;

#[derive(Debug, Default)]
pub struct ExactResolver;

#[derive(Debug)]
pub enum ExactResolverError {
    TooManyPossibilities {
        request: TypeInfoWithRustc,
        possibles: Vec<TypeInfoWithRustc>,
    },
}

impl super::RegistryConflictResolver for ExactResolver {
    type Error = ExactResolverError;

    fn check(&self, type_request: &TypeInfoWithRustc, type_info: &TypeInfoWithRustc) -> bool {
        type_request == type_info
    }

    fn resolve<'a, T>(
        &self,
        type_request: &TypeInfoWithRustc,
        possibles: Vec<(&'a TypeInfoWithRustc, &T)>,
    ) -> Result<Option<&'a T>, Self::Error> {
        Err(ExactResolverError::TooManyPossibilities {
            request: type_request.clone(),
            possibles: possibles.into_iter().map(|x| x.0.clone()).collect(),
        })
    }
}
