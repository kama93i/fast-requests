import fast_requests

urls = [
    "https://google.com",
    "https://wikipedia.org",
    "https://httpbin.org/status/404",  # Will be an error
]

results = fast_requests.get_many(urls)

for result in results:
    if hasattr(result, 'text'):  # It's a PyResponse
        print(f"✓ {result.status_code} - {result.url}")
        print(f"  Body preview: {result.text[:100]}...")
    else:  # It's a PyRequestError
        print(f"✗ Error {result.status_code}: {result.error}")
