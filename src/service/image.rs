use base64::{engine::general_purpose, Engine as _};
use chrono::prelude::*;
use cloud_storage::Object;
use futures::{stream, StreamExt};

use super::error::{ErrorWrapper, FmtError};

use super::dtm::image::dto::ImageCreateDto;

use super::aggregation::image::ImageAggregation;

const IMAGE_BUCKET: &str = "wiki-content-images";

pub struct ImageService;

impl ImageService {
    pub async fn upload_images(
        creation_dtos: Vec<ImageCreateDto>,
    ) -> Result<Vec<ImageAggregation>, ErrorWrapper> {
        let responses = stream::iter(creation_dtos)
            .map(move |image_dto| {
                tokio::spawn(async move {
                    let timestamp = Utc::now().timestamp_millis().to_string();

                    let filename = format!("{}_{}.{}", timestamp, image_dto.id, image_dto.format);
                    let binary_str = general_purpose::STANDARD
                        .decode(image_dto.base64)
                        .expect(&FmtError::FailedToProcess("image base64").fmt());

                    let object = Object::create(
                        IMAGE_BUCKET,
                        binary_str,
                        filename.as_str(),
                        format!("image/{}", image_dto.format).as_str(),
                    )
                    .await;

                    (object, image_dto.id)
                })
            })
            .buffer_unordered(10);

        let images_objects = responses
            .fold(vec![], |mut images_objects, response_dto| async move {
                let (result, id) = response_dto.expect(&FmtError::FailedToInsert("image").fmt());
                let upload_result = result.expect(&FmtError::FailedToInsert("image").fmt());

                images_objects.push(ImageAggregation {
                    id,
                    uri: format!(
                        "https://storage.googleapis.com/wiki-content-images/{}",
                        upload_result.name
                    ),
                });

                images_objects
            })
            .await;

        Ok(images_objects)
    }
}
