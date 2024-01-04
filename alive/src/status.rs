pub fn status() -> &'static str {
  "SELECT kind_id,host_id,dns_type,err FROM watch ORDER BY err DESC,kind_id,host_id"
}
