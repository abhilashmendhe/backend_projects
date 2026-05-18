
lb_index = 0

async def lb():
    global lb_index
    ports = [58233, 58234, 58235]
    port = ports[lb_index]
    lb_index = (lb_index + 1) % len(ports)
    return port