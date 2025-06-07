CREATE TABLE IF NOT EXISTS performance_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    component_name VARCHAR(100) NOT NULL,
    metric_type VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,6) NOT NULL,
    unit_of_measure VARCHAR(50),
    context_data JSONB DEFAULT '{}',
    threshold_data JSONB DEFAULT '{}',
    alert_level VARCHAR(20) CHECK (alert_level IN ('info', 'warning', 'critical')),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_performance_metrics_component_name ON performance_metrics(component_name);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_metric_type ON performance_metrics(metric_type);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_timestamp ON performance_metrics(timestamp);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_alert_level ON performance_metrics(alert_level);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_composite ON performance_metrics(component_name, metric_type, timestamp);
