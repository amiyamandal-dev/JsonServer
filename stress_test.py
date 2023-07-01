import asyncio
import aiohttp
import time


# Define the API endpoint URL
api_url = "http://localhost:9777/"

# Define the number of concurrent requests to send
num_requests = 10000000

# Define the payload or parameters to send with each request (if applicable)
# payload = {
#     "param1": "value1",
#     "param2": "value2"
# }

# Define an async function that sends a single request
async def send_request(session):
    try:
        start_time = time.time()  # Record the start time
        async with session.get(api_url) as response:
            # Process the response as needed
            print(response.status)
        end_time = time.time()  # Record the end time
        execution_time = end_time - start_time
        print(f"Request executed in {execution_time:.2f} seconds")
    except aiohttp.ClientError as e:
        print("Error:", e)

# Create an async function to send concurrent requests
async def stress_test():
    async with aiohttp.ClientSession() as session:
        tasks = []
        for _ in range(num_requests):
            task = asyncio.ensure_future(send_request(session))
            tasks.append(task)
        await asyncio.gather(*tasks)

# Run the stress test
loop = asyncio.get_event_loop()
loop.run_until_complete(stress_test())
