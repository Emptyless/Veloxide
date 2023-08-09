# Supporting Containers

Veloxide comes pre-configured with the following supporting containers found in the `docker-compose.yml` file:

- **[Tempo](https://grafana.com/oss/tempo/)**: Traces will be sent to Tempo using OTLP, which can be accessed at `http://localhost:3000/explore?`.
- **[Prometheus](https://prometheus.io/)** will be available at `http://localhost:9090`.
- **[Grafana](https://grafana.com/)**: Will be available at `http://localhost:3000`. The default username and password are both `admin`. Prometheus is already configured as the default data source.
- **[Postgres](https://www.postgresql.org/)** will be be listening on port `5432` for new connections. The connection string is loaded from the environment variable `DATABASE_URL`, which is pre-configured in the .env file.
- Open Policy Agent (OPA) will be available at `http://localhost:8181`. The policies are loaded from the `policies` directory.
