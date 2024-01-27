# fm.loige.co

The _Rusty_ serverless API that powers [fm.loige.co](https://fm.loige.co/playing). Developed with [AWS Lambda](https://aws.amazon.com/lambda/), [Rust](https://www.rust-lang.org/) and [SAM](https://aws.amazon.com/serverless/sam/).

If you want to build something like this for yourself, you can use this repo as a reference and follow the instructions below.



## Requirements

It requires a [Last.fm](https://www.last.fm/) account and a [Last.fm API key](https://www.last.fm/api/account/create).

If you want to deploy it on AWS you need an AWS account and the [AWS CLI](https://aws.amazon.com/cli/) installed and configured.

You'll also need the Rust toolchain, [cargo-lambda](https://www.cargo-lambda.info/), [Docker](https://www.docker.com/), and SAM installed on your machine.


## Usage

First of all, grab an API key and make sure to populate the `LASTFM_API_KEY` environment variable with it.

### Running locally

To run the Lambda locally you can use `cargo lambda`:

```bash
$ cargo lambda watch
```

This will spin up a development server to which you can send requests. For example:

```bash
$ cargo lambda invoke --data-example apigw-request
```

## Deploying on AWS

Before deploying to AWS you most likely want to customise the [`template.yaml`](/template.yaml).

In particular, you will want to change all the various parameters. I am also using a custom domain name, so you'll want to change that as well.

Domain name requires validation, so when the first deployment starts, it will stay in a _pending_ state until you create the necessary DNS records to validate the TLS certificate. This can be done manually or by logging in the AWS console and going to the Certificate Manager service.

> **Note**: This is only necessary for the first deployment

Once you are happy with the configuration, you can deploy the stack with:

```bash
$ sam deploy --guided
```

And follow the guided procedure.

## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/fm.loige.co/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.

