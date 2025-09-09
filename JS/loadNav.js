// File name: oadNav.js
// Dynamically loads the correct navigation bar based on a data attribute
// and attaches mobile dropdown behavior

document.addEventListener('DOMContentLoaded', function() {
  const navSlots = document.querySelectorAll('.nav-slot');

  // 🔒 SECURITY: Whitelist of allowed nav types with updated paths
  const allowedNavTypes = {
    "javascript": "/Nav/Nav_JS.html",
    "css-html": "/Nav/Nav_CSS_HTML.html",
    "index": "/Nav/Nav_Index.html",
    "Rust": "/Nav/Nav_Rust.html"
  };

  navSlots.forEach(slot => {
    const navType = slot.dataset.navType; // "javascript", "css-html", or "index"
    const path = allowedNavTypes[navType];

    if (!path) {
      console.error(`🔒 No navigation file defined for type: ${navType}`);
      return;
    }

    // Fetch and insert the nav HTML
    fetch(path)
      .then(res => res.text())
      .then(html => {
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');
        const template = doc.querySelector('template');

        if (template) {
          slot.appendChild(template.content.cloneNode(true));
          attachMobileDropdown(slot);
        } else {
          console.warn(`No <template> found in ${path}`);
        }
      })
      .catch(err => console.error(`Error loading nav: ${err}`));
  });
});

// Mobile/iPhone dropdown behavior
function attachMobileDropdown(container) {
  const dropdowns = container.querySelectorAll('.dropdown');

  dropdowns.forEach(drop => {
    const toggle = drop.querySelector('a');

    toggle.replaceWith(toggle.cloneNode(true));
    const newToggle = drop.querySelector('a');

    newToggle.addEventListener('click', function(e) {
      e.preventDefault();

      dropdowns.forEach(d => {
        if (d !== drop) d.classList.remove('active');
      });

      drop.classList.toggle('active');

      if (drop.classList.contains('active')) {
        document.body.classList.add('dropdown-open');
      } else {
        document.body.classList.remove('dropdown-open');
      }
    });
  });

  document.addEventListener('click', function(e) {
    dropdowns.forEach(drop => {
      if (!drop.contains(e.target)) {
        drop.classList.remove('active');
        document.body.classList.remove('dropdown-open');
      }
    });
  });
}

/* chatGPT Questions by liam Disregard if I forgot to remove it. 





*/