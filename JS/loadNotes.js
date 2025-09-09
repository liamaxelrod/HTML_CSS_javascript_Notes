// File name: oadNotes.js
// Base folder path where all notes HTML files are stored
const notesFolder = '/N';

// 🔒 SECURITY: Whitelist allowed folders with updated names
const allowedFolders = ['NHTML', 'NCSS', 'NJS', 'NRust'];

// 🔒 SECURITY: Whitelist allowed note IDs for each folder (updated)
const allowedNotes = {
  'NHTML': [
    'Basics',
    'Forms_Inputs',
    'JS_Integration',
    'Structure',
    'Text_Links'
  ],
  'NCSS': [
    'Animation',
    'Basics',
    'Categories',
    'Icons_Links',
    'Layout',
    'Properties'
  ],
  'NJS': [
    'Basics',
    'Data',
    'Flow_Logic',
    'Functions_Classes',
    'Math',
    'Errors',
    'Browser'
  ],
  'NRust': [
    'Basics'
  ]
};

// Main function to load notes into placeholders on the page
async function loadNeededNotes() {
  const slots = document.querySelectorAll('.note-slot[data-id]');
  console.log(`Found ${slots.length} note slots to fill`);

  for (const slot of slots) {
    const folder = slot.getAttribute('data-folder') || '';
    const noteId = slot.getAttribute('data-id');

    if (!allowedFolders.includes(folder)) {
      console.warn(`🔒 Skipped note: Folder not allowed -> ${folder}`);
      continue;
    }

    if (!allowedNotes[folder]?.includes(noteId)) {
      console.warn(`🔒 Skipped note: Note ID not allowed -> ${noteId}`);
      continue;
    }

    let path = folder ? `${notesFolder}/${folder}/${noteId}.html` : `${notesFolder}/${noteId}.html`;

    try {
      const res = await fetch(path);

      if (!res.ok) {
        console.warn(`Could not load ${path}: ${res.statusText}`);
        continue;
      }

      const html = await res.text();
      const parser = new DOMParser();
      const doc = parser.parseFromString(html, 'text/html');
      const templates = doc.querySelectorAll('template');

      if (templates.length === 0) {
        console.warn(`No <template> found in ${path}`);
        continue;
      }

      templates.forEach(template => {
        slot.appendChild(template.content.cloneNode(true));
      });

    } catch (error) {
      console.error(`Error loading ${path}:`, error);
    }
  }
}

// Start loading all needed notes
loadNeededNotes();

/* chatGPT Questions by liam Disregard if I forgot to remove it. 





*/