document.getElementById('aiForm').addEventListener('submit', async function (event) {
  event.preventDefault();
  const prompt = document.getElementById('prompt').value;
  const responseDiv = document.getElementById('response');
  const button = event.target.querySelector('button');

  // Disable the button and change the text
  button.disabled = true;
  button.textContent = "Working on it...";
  try {
    const response = await fetch('/api/infer', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ question: prompt }),
    });

    if (!response.ok) {
      throw new Error('Network response was not ok');
    }

    const data = await response.text();
    responseDiv.textContent = data;
  } catch (error) {
    responseDiv.textContent = 'An error occurred: ' + error.message;
  } finally {
    button.disabled = false;
    button.textContent = "Ask";
  }
});

