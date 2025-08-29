// loadNav.js
// Dynamically loads the correct navigation bar based on a data attribute
// and attaches mobile dropdown behavior

document.addEventListener('DOMContentLoaded', function() {
  const navSlots = document.querySelectorAll('.nav-slot');

  navSlots.forEach(slot => {
    // Get desired nav type from the HTML element
    const navType = slot.dataset.navType; // "javascript" or "css-html"

    // Map nav types to file paths
    const navPaths = {
      "javascript": "/Navigation_Bar/Nav_Javascript.html",
      "css-html": "/Navigation_Bar/Nav_CSS_HTML.html",
      "index": "/Navigation_Bar/Nav_Index.html"
    };

    const path = navPaths[navType];

    if (!path) {
      console.error(`No navigation file defined for type: ${navType}`);
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
          attachMobileDropdown(slot); // attach dropdown after load
        }
      })
      .catch(err => console.error('Error loading nav:', err));
  });
});

// Mobile/iPhone dropdown behavior
function attachMobileDropdown(container) {
  const dropdowns = container.querySelectorAll('.dropdown');

  dropdowns.forEach(drop => {
    const toggle = drop.querySelector('a');

    // Remove existing listener if re-attaching
    toggle.replaceWith(toggle.cloneNode(true));
    const newToggle = drop.querySelector('a');

    newToggle.addEventListener('click', function(e) {
      e.preventDefault();

      // Close all other dropdowns in this nav
      dropdowns.forEach(d => {
        if (d !== drop) d.classList.remove('active');
      });

      // Toggle this dropdown
      drop.classList.toggle('active');

      // Prevent body scroll when dropdown is open
      if (drop.classList.contains('active')) {
        document.body.classList.add('dropdown-open');
      } else {
        document.body.classList.remove('dropdown-open');
      }
    });
  });

  // Close dropdown if clicking outside of nav
  document.addEventListener('click', function(e) {
    dropdowns.forEach(drop => {
      if (!drop.contains(e.target)) {
        drop.classList.remove('active');
        document.body.classList.remove('dropdown-open');
      }
    });
  });
}

