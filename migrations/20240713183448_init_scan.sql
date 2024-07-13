-- Create ScanStatus type
CREATE TYPE ScanStatus AS ENUM ('pending', 'running', 'finished', 'error');

-- Create Scan table
CREATE TABLE scans (
    id SERIAL PRIMARY KEY,
    host VARCHAR(255) NOT NULL,
    status ScanStatus NOT NULL,
    scan_ports INTEGER[] NULL,
    exclude_ports INTEGER[] NULL,
    timeout BIGINT NULL,
    start_port INTEGER NULL,
    end_port INTEGER NULL,
    udp BOOLEAN NULL,
    tcp BOOLEAN NULL,
    open_ports INTEGER[] NULL
);
