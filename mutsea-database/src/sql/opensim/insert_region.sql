INSERT INTO regions (
    uuid, region_name, server_ip, server_port, 
    loc_x, loc_y, size_x, size_y, last_seen
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);