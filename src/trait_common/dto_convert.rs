pub trait DtoConvert<TDto> {
    type TParams;

    fn into_dto(self, params: Self::TParams) -> TDto
    where
        Self: Sized;
}
