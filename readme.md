docker-network-test
===================

Very simple container, contains a Rust binary that tries to resolve cloudflare.com using Cloudflare's own public DNS service.


Returns 1 on stdout when it thinks the resolve was successful.

Returns 0 on stdout in any other case.



Why does this exist?
--------------------

I've had some issues with Docker containers not having network connectivity (for whatever reason), 
and I'd get notified of this early instead of via failed CI builds.

This container makes it easy to do simple network test from within a Docker container, and have the output parsed by Zabbix Agent on the host, via a UserParameter.
