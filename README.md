# volume-init-s3-objects
Initialize the container volume by downloading S3 objects.
This utility is intended to be used with features like Init Containers in Kubernetes.

## Usage

1. Set the environment variable `VOLUME_INIT_S3_OBJECTS_CONFIG` with the JSON configuration.
2. Run the container with the volume mounted you want to initialize.

## Configuration

- `timeOutSec`: Optional. Timeout in seconds for each file download. Default is 15 seconds.
- `retryCountPerFile`: Optional. Number of retry attempts for each file download. Default is 3 attempts.
- `files`: A map where the key is the file path and the value is an object containing:
  - `bucket`: The S3 bucket name.
  - `key`: The S3 object key.
  - `versionId`: Optional. The version ID of the S3 object.

## Example

```sh
export AWS_REGION=ap-northeast-1
export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
export VOLUME_INIT_S3_OBJECTS_CONFIG='{
  "timeOutSec": 20,
  "retryCountPerFile": 5,
  "files": {
    "/tmp/file1": {
      "bucket": "example-bucket",
      "key": "example/key1"
    },
    "/tmp/file2": {
      "bucket": "example-bucket",
      "key": "example/key2",
      "versionId": "example-version-id"
    }
  }
}'
docker run --rm \
  --env AWS_REGION \
  --env AWS_ACCESS_KEY_ID \
  --env AWS_SECRET_ACCESS_KEY \
  --env VOLUME_INIT_S3_OBJECTS_CONFIG \
  --volume /tmp:/tmp \
  ghcr.io/hrko/volume-init-s3-objects:latest
```

