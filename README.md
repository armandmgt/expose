# Expose

Expose allows you to access local web applications (or any port) from the internet. Additionally, it provides a dashboard that enables you to inspect and replay HTTP requests made to your local applications. With this tool, you can easily share your locally hosted web applications with others for testing, collaboration, or demonstrations.

## Features

- Expose local web applications to the internet: The reverse proxy enables you to make your locally hosted web applications accessible from anywhere, without deploying them to a public server.

- Web dashboard for request inspection: The included web dashboard provides a user-friendly interface to inspect incoming requests, including request headers, parameters, and payloads.

- Request replay functionality: The dashboard also allows you to replay requests, making it convenient for testing and debugging purposes.

## Getting Started

To get started with expose you will need a server accessible from internet. Expose is self-hosted only so that you are the only owner of your data.

First configure the server by following these steps:

1. Clone the repository on your server:

```bash
git clone https://github.com/armandmgt/expose
```

2. Configure the reverse proxy:

Create a `conf/production.json` file based on `conf/default.json` and specify the differents variables.

3. Start the reverse proxy:

```bash
docker-compose up -d
```

## Contributing

Contributions are welcome! If you find any issues or have ideas for improvements, please open an issue or submit a pull request. Make sure to follow the project's code of conduct.

## License

This project is licensed under the [MIT License](./LICENSE). Feel free to use, modify, and distribute the code as per the terms of the license.

## Acknowledgements

This project was inspired by the functionalities provided by [ngrok](https://github.com/ngrok) and [pgrok](https://github.com/pgrok/pgrok) and aims to provide a similar solution for exposing local web applications. We would like to thank the ngrok and pgrok team for their excellent work.
