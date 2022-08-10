import contextlib
import asyncio


async def stop_database():
    await asyncio.sleep(2.0)
    print("systemctl stop postgresql.service")


async def start_database():
    await asyncio.sleep(1.0)
    print("systemctp start postgresql.service")


@contextlib.asynccontextmanager
async def db_management():
    """
    stop database, then backup the database
    and start it up again after the backup
    """
    try:
        await stop_database()
        yield
    finally:
        await start_database()


async def create_metrics_logger():
    await asyncio.sleep(0.01)
    return "metrics-logger"


@contextlib.asynccontextmanager
async def metrics_logger():
    yield await create_metrics_logger()


async def run_db_backup():
    async with db_management(), metrics_logger():
        print("Performing DB backup...")


async def main():
    await run_db_backup()


if __name__ == "__main__":
    val = asyncio.run(main())
