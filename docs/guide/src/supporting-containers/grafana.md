# Grafana

## Introduction to Grafana

Grafana is a popular open-source data visualization and monitoring tool that provides a wide range of benefits and value propositions for users. Here are some of the key benefits of using Grafana:

- Data Visualization: Grafana provides a flexible and intuitive interface for creating and customizing data visualizations, allowing users to easily create charts, graphs, dashboards, and other visualizations that can help them gain insights into their data.

- Integrations: Grafana supports a wide range of data sources, including popular databases like MySQL, PostgreSQL, and Microsoft SQL Server, as well as cloud services like AWS CloudWatch, Google Cloud Platform, and Microsoft Azure. This makes it easy for users to pull data from multiple sources and create unified dashboards and visualizations.

- Dashboard Sharing and Collaboration: Grafana allows users to easily share and collaborate on dashboards with other users, either within the organization or with external stakeholders. This can help teams to work more effectively together and ensure that everyone has access to the same data and insights.

- Alerting and Notifications: Grafana includes a built-in alerting system that allows users to set up alerts based on specific metrics or thresholds, and receive notifications via email, Slack, or other channels when those alerts are triggered. This can help teams to quickly identify and respond to issues before they become critical.

- Extensibility: Grafana is highly extensible, with a robust plugin architecture that allows users to add new features and functionality to the platform. This makes it possible to customize Grafana to meet specific business requirements and use cases.

Overall, Grafana provides a powerful and flexible platform for data visualization and monitoring, allowing users to gain deep insights into their data, collaborate effectively with others, and quickly identify and respond to issues as they arise.

## Veloxide and Grafana

The Grafana instance spun up by docker compose is configured to use Prometheus as a datasource, which in turn collects the metrics from /metrics. The Grafana instance is configured to use the Prometheus datasource by default. At the time of writing, there are no dashboards configured by default, but you can create your own however.
