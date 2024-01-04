"SELECT kind.v, dns_type, host.v, err FROM watch, kind, host WHERE kind_id = kind.id AND watch.host_id = host.id ORDER BY err DESC, kind_id, host_id"
