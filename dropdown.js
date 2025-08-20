// dropdown.js
// Handles mobile dropdown toggling and outside click closing

document.addEventListener('DOMContentLoaded', function() {
  // Select all dropdown containers
  const dropdowns = document.querySelectorAll('.dropdown');

  dropdowns.forEach(drop => {
    const toggle = drop.querySelector('a'); // the link that toggles dropdown

    toggle.addEventListener('click', function(e) {
      e.preventDefault(); // prevent default link action

      // Close all other dropdowns
      dropdowns.forEach(d => {
        if (d !== drop) d.classList.remove('active');
      });

      // Toggle current dropdown
      drop.classList.toggle('active');
    });
  });

  // Close dropdowns if clicking outside
  document.addEventListener('click', function(e) {
    dropdowns.forEach(drop => {
      if (!drop.contains(e.target)) {
        drop.classList.remove('active');
      }
    });
  });
});