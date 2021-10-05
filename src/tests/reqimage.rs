use crate::reqimage::RequestedImage;
use crate::utils::get_static_file;
use std::fs;
use std::path::Path;

#[test]
#[ignore]
fn valid_request_static_image() {
    // remove old file if exists
    let old_file = get_static_file("rustybucket_35.png");
    if old_file.is_file() {
        fs::remove_file(&old_file).ok();
    }

    let path = Path::new("rustybucket.png");

    let img = RequestedImage::new(path, 35, None, false);

    // original image content is a png
    assert!(img.content_type.is_some());

    // original image exists
    assert!(img.path.is_file());

    // requested image doesn't exist yet
    assert!(!img.exists());

    // save image and check that it exists on disk
    assert!(img.save().is_ok());
    assert!(img.exists());

    // read and encode file
    assert!(img.read().is_ok());

    fs::remove_file(old_file).ok();
}

#[test]
#[ignore]
fn valid_request_static_image_new_ext() {
    // remove old file if exists
    let old_file = get_static_file("rustybucket_10.png");
    if old_file.is_file() {
        fs::remove_file(&old_file).ok();
    }
    let old_file_webp = get_static_file("rustybucket_10.webp");
    if old_file_webp.is_file() {
        fs::remove_file(&old_file_webp).ok();
    }

    let path = Path::new("rustybucket.png");

    let img = RequestedImage::new(path, 10, Some("webp"), false);

    // original image content is a png
    assert!(img.content_type.is_some());

    // original image exists
    assert!(img.path.is_file());

    // requested image doesn't exist yet
    assert!(!img.exists());

    // save image and check that it exists on disk
    assert!(img.save().is_ok());
    assert!(img.exists());

    // read and encode file
    assert!(img.read().is_ok());

    fs::remove_file(old_file).ok();
    fs::remove_file(old_file_webp).ok();
}

#[test]
fn valid_request_public_image() {
    let path = Path::new("favicon.ico");

    let img = RequestedImage::new(path, 0, None, true);

    // original image content is a png
    assert!(img.content_type.is_some());

    // original image exists
    assert!(img.path.is_file());

    // read and encode file
    assert!(img.read().is_ok());
}

#[test]
fn invalid_request_image() {
    let path = Path::new("malformedimageext.p");

    let img = RequestedImage::new(path, 0, None, false);

    // original image content is a png
    assert!(img.content_type.is_none());

    // original image exists
    assert!(!img.path.is_file());

    assert!(img.save().is_err());
}
