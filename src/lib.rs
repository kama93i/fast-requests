use futures::future::join_all;
use pyo3::prelude::*;
use std::time::Duration;
use tokio::task;

// Python-visible Response type
#[pyclass]
#[derive(Clone)]
struct PyResponse {
    #[pyo3(get)]
    status_code: u16,

    #[pyo3(get)]
    text: String,

    #[pyo3(get)]
    url: String,
}

// Python-visible Error type
#[pyclass]
#[derive(Clone)]
struct PyRequestError {
    #[pyo3(get)]
    status_code: u16,

    #[pyo3(get)]
    error: String,
}

// Internal Rust error (not exposed to Python)
#[derive(Debug)]
struct RequestError {
    status_code: u16,
    error: String,
}

async fn fetch_multiple(urls: Vec<String>) -> Vec<Result<reqwest::Response, RequestError>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let mut handles = vec![];

    for url in urls {
        let client = client.clone();
        let handle = task::spawn(async move { client.get(&url).send().await });
        handles.push(handle);
    }

    let results = join_all(handles).await;
    let mut responses = vec![];

    for result in results {
        let res = match result {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(http_error)) => Err(RequestError {
                status_code: http_error.status().map(|s| s.as_u16()).unwrap_or(0),
                error: http_error.to_string(),
            }),
            Err(join_error) => Err(RequestError {
                status_code: 0,
                error: join_error.to_string(),
            }),
        };
        responses.push(res);
    }

    responses
}

#[pyfunction]
fn get_many(py: Python, urls: Vec<String>) -> PyResult<Vec<PyObject>> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let python_results = runtime.block_on(async {
        let responses = fetch_multiple(urls).await;
        let mut results = vec![];

        for result in responses {
            match result {
                Ok(response) => {
                    // Extract data from response
                    let status = response.status().as_u16();
                    let url = response.url().to_string();
                    let text = response.text().await.unwrap_or_default();

                    // Create Python object
                    let py_response = PyResponse {
                        status_code: status,
                        text,
                        url,
                    };

                    // Convert to PyObject
                    let py_obj = Py::new(py, py_response).unwrap().into_py(py);
                    results.push(py_obj);
                }
                Err(e) => {
                    // Create Python error object
                    let py_error = PyRequestError {
                        status_code: e.status_code,
                        error: e.error,
                    };

                    // Convert to PyObject
                    let py_obj = Py::new(py, py_error).unwrap().into_py(py);
                    results.push(py_obj);
                }
            }
        }

        results
    });

    Ok(python_results)
}

#[pymodule]
fn fast_requests(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_many, m)?)?;
    m.add_class::<PyResponse>()?;
    m.add_class::<PyRequestError>()?;
    Ok(())
}

