import { NodeSDK } from '@opentelemetry/sdk-node';
import { getNodeAutoInstrumentations } from '@opentelemetry/auto-instrumentations-node';
import { PeriodicExportingMetricReader } from '@opentelemetry/sdk-metrics';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { OTLPMetricExporter } from '@opentelemetry/exporter-metrics-otlp-proto';

const TRACE_URL = import.meta.env.VITE_TRACE_URL || 'http://localhost:4318/v1/traces';
const METRICS_URL = import.meta.env.VITE_METRICS_URL || 'http://localhost:4318/v1/metrics';

const otelNodeSdk = new NodeSDK({
	traceExporter: new OTLPTraceExporter({
		url: TRACE_URL,
		headers: {}
	}),
	metricReader: new PeriodicExportingMetricReader({
		exporter: new OTLPMetricExporter({
			url: METRICS_URL,
			headers: {}
		})
	}),
	instrumentations: [getNodeAutoInstrumentations()]
});

export class Telemetry {
	private static instance: Telemetry;
	private initialized = false;

	private constructor() {}

	public static getInstance(): Telemetry {
		if (!Telemetry.instance) {
			Telemetry.instance = new Telemetry();
		}
		return Telemetry.instance;
	}

	public start() {
		if (!this.initialized) {
			this.initialized = true;
			otelNodeSdk.start();
		}
	}
}
