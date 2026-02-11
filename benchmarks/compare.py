import time
import requests
import fast_requests

def benchmark_requests(urls, iterations=3):
    """Benchmark using requests library (sequential)"""
    times = []
    
    for _ in range(iterations):
        start = time.time()
        results = []
        for url in urls:
            try:
                response = requests.get(url, timeout=30)
                results.append(response)
            except Exception as e:
                results.append(None)
        elapsed = time.time() - start
        times.append(elapsed)
    
    avg_time = sum(times) / len(times)
    return avg_time, len([r for r in results if r])

def benchmark_fast_requests(urls, iterations=3):
    """Benchmark using fast_requests (concurrent)"""
    times = []
    
    for _ in range(iterations):
        start = time.time()
        results = fast_requests.get_many(urls)
        elapsed = time.time() - start
        times.append(elapsed)
    
    avg_time = sum(times) / len(times)
    success_count = sum(1 for r in results if hasattr(r, 'text'))
    return avg_time, success_count

def main():
    # Test with different URL counts
    test_cases = [
        ("10 URLs", [
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/1",
        ]),
        ("20 URLs", [f"https://httpbin.org/delay/1" for _ in range(20)]),
        ("50 URLs", [f"https://httpbin.org/delay/1" for _ in range(50)]),
    ]
    
    print("=" * 70)
    print("fast-requests vs requests - Performance Benchmark")
    print("=" * 70)
    print()
    
    for test_name, urls in test_cases:
        print(f"Test: {test_name}")
        print("-" * 70)
        
        # Benchmark requests
        print("Running requests (sequential)...", end=" ", flush=True)
        req_time, req_success = benchmark_requests(urls)
        print(f"Done! {req_time:.2f}s (avg)")
        
        # Benchmark fast_requests
        print("Running fast_requests (concurrent)...", end=" ", flush=True)
        fast_time, fast_success = benchmark_fast_requests(urls)
        print(f"Done! {fast_time:.2f}s (avg)")
        
        # Calculate speedup
        speedup = req_time / fast_time
        
        print()
        print(f"Results:")
        print(f"  requests:       {req_time:.2f}s ({req_success} successful)")
        print(f"  fast_requests:  {fast_time:.2f}s ({fast_success} successful)")
        print(f"  Speedup:        {speedup:.1f}x faster! 🚀")
        print()
        print()

if __name__ == "__main__":
    main()
