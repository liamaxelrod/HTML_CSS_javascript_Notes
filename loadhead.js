// File name: loadhead.js
// Dynamically loads a head template into the <head> tag

document.addEventListener('DOMContentLoaded', () => {
  const head = document.querySelector('head');

  // Select template based on data attribute (default to "default")
  const templateName = head.dataset.headTemplate || 'default';

  // Only fetch from local /head folder → secure
  const path = `/head/${templateName}.html`;

  fetch(path)
    .then(res => {
      if (!res.ok) throw new Error(`Failed to load head template: ${res.status}`);
      return res.text();
    })
    .then(html => head.insertAdjacentHTML('beforeend', html))
    .catch(err => console.error(err));
});

 