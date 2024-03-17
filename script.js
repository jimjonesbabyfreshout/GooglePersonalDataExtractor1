const form = document.getElementById("form");
const fileInput = document.getElementById("file-input");
const textView = document.getElementById("text-viewer");
const buttons = document.getElementById("buttons");
const copyButton = document.getElementById("copy-button");
const saveButton = document.getElementById("save-button");

form.addEventListener("submit", (e) => {
  e.preventDefault();

  const file = fileInput.files[0];

  const reader = new FileReader();

  reader.onload = () => {
    const text = reader.result;

    const data = extractData(text);

    textView.textContent = data;
  };

  reader.readAsText(file);
});

copyButton.addEventListener("click", () => {
  const text = textView.textContent;

  navigator.clipboard.writeText(text);
});

saveButton.addEventListener("click", () => {
  const text = textView.textContent;

  const blob = new Blob([text], { type: "text/plain" });

  const url = URL.createObjectURL(blob);

  const a = document.createElement("a");

  a.href = url;
  a.download = "google-personal-data.txt";

  a.click();

  URL.revokeObjectURL(url);
});

function extractData(text) {
  // Extract the data from the HTML file.

  return data;
}