const notesFolder = '/All_Notes';  // Base folder path where all notes HTML files are stored

async function loadNeededNotes() {
  // Find all elements on the page that have class "note-slot" and a data-id attribute
  // These are placeholders where note templates will be loaded
  const slots = document.querySelectorAll('.note-slot[data-id]');
  console.log(`Found ${slots.length} note slots to fill`);  // Log how many slots were found

  // Loop through each placeholder slot found on the page
  for (const slot of slots) {
    // Get the optional folder name from the data-folder attribute or use empty string if none
    const folder = slot.getAttribute('data-folder') || '';  
    
    // Get the note ID from the data-id attribute, used to identify which note to load
    const noteId = slot.getAttribute('data-id');
    
    // Build the path to the note's HTML file
    // If a folder is specified, include it in the path; otherwise just use the noteId
    let path;
    if (folder) {
      path = `${notesFolder}/${folder}/${noteId}.html`;
    } else {
      path = `${notesFolder}/${noteId}.html`;
    }

    // Log the path being fetched for debugging purposes
    try {
      // Fetch the note HTML file from the constructed path
      const res = await fetch(path);

      // If the HTTP response is not OK, log a warning and skip to the next slot
      if (!res.ok) {
        console.warn(`Could not load ${path}: ${res.statusText}`);
        continue;
      }

      // Extract the response body as text (HTML content)
      const html = await res.text();

      // Parse the fetched HTML string into a new Document object
      const parser = new DOMParser();
      const doc = parser.parseFromString(html, 'text/html');
      
      // Look for all <template> elements inside the fetched HTML document
      const templates = doc.querySelectorAll('template');

      // If no <template> is found, warn and skip this slot
      if (templates.length === 0) {
        console.warn(`No <template> found in ${path}`);
        continue;
      }

      // Clone the content of all templates and append them to the current placeholder slot
      // This allows loading multiple templates sequentially from a single note file
      templates.forEach(template => {
        slot.appendChild(template.content.cloneNode(true));
      });

    } catch (error) {
      // If an error occurs during fetching or parsing, log it to the console
      console.error(`Error loading ${path}:`, error);
    }
  }
}

// Start loading all needed notes as soon as this script runs
loadNeededNotes();

/* chatGPT Questions by liam Disregard if I forgot to remove it.  

*/
