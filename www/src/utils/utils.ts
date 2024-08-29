export async function postData(url: string, data: unknown) {
  return fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then((response) => response.json());
}

export async function getData(url: string) {
  return fetch(url).then((response) => response.json());
}