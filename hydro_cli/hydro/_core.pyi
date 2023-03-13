from typing import AsyncGenerator, List, Optional

class Deployment(object):
    def __init__(self) -> None: ...

    def Localhost(self) -> "LocalhostHost": ...

    def GCPComputeEngineHost(self, project: str, machine_type: str, image: str, region: str) -> "GCPComputeEngineHost": ...

    def CustomService(self, on: "Host", external_ports: List[int]) -> "CustomService": ...

    def HydroflowCrate(self, src: str, on: "Host", example: Optional[str] = None, features: Optional[List[str]] = None, args: Optional[List[str]] = None) -> "HydroflowCrate": ...

    async def deploy(self): ...

    async def start(self): ...

class Host(object):
    pass

class LocalhostHost(Host):
    pass

class GCPComputeEngineHost(Host):
    internal_ip: str
    external_ip: Optional[str]
    ssh_key_path: str

class Service(object):
    pass

class CustomService(Service):
    pass

class HydroflowCrate(Service):
    ports: HydroflowCratePorts
    async def stdout(self) -> AsyncGenerator[str, None]: ...
    async def stderr(self) -> AsyncGenerator[str, None]: ...
    async def exit_code(self) -> int: ...

class HydroflowCratePorts(object):
    def __getattribute__(self, __name: str) -> HydroflowCratePort: ...

class HydroflowCratePort(object):
    def send_to(self, other: HydroflowCratePort) -> None: ...
