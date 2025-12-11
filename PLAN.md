# Multi-Board Landing Page Implementation Plan

## Overview
Add a landing page that displays all boards with CRUD operations, replacing the current single-board auto-load behavior.

---

## Current State
- Single board auto-loads from `~/.local/share/humanboard/board.json`
- `Board` struct contains items, zoom, offset, history
- `Humanboard` component is the root, directly contains `Board`
- No concept of multiple boards or board metadata

---

## Proposed Architecture

### New Data Structures

```rust
// Board metadata (stored in index file)
#[derive(Serialize, Deserialize, Clone)]
pub struct BoardMetadata {
    pub id: String,           // UUID
    pub name: String,         // User-facing name
    pub created_at: u64,      // Unix timestamp
    pub updated_at: u64,      // Unix timestamp
    pub thumbnail: Option<PathBuf>, // Preview image (optional, future)
}

// Index of all boards
#[derive(Serialize, Deserialize)]
pub struct BoardIndex {
    pub boards: Vec<BoardMetadata>,
}
```

### Storage Structure
```
~/.local/share/humanboard/
├── index.json              # BoardIndex - list of all boards
└── boards/
    ├── {uuid1}.json        # BoardState for board 1
    ├── {uuid2}.json        # BoardState for board 2
    └── ...
```

### App State Machine

```rust
pub enum AppView {
    Landing,              // Shows board grid with CRUD
    Board(String),        // Active board by ID
}

pub struct Humanboard {
    pub view: AppView,
    pub board_index: BoardIndex,
    // ... existing board-related fields (only populated when view is Board)
    pub board: Option<Board>,
    // ...
}
```

---

## UI Design

### Landing Page Layout
```
┌─────────────────────────────────────────────────────────────┐
│  Humanboard                                    [+ New Board] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌───────────┐  ┌───────────┐  ┌───────────┐              │
│   │           │  │           │  │           │              │
│   │  Board 1  │  │  Board 2  │  │  Board 3  │              │
│   │           │  │           │  │           │              │
│   ├───────────┤  ├───────────┤  ├───────────┤              │
│   │ Project X │  │ Mood 2024 │  │ Ideas     │              │
│   │ Dec 10    │  │ Dec 8     │  │ Dec 5     │              │
│   │ [✎] [🗑]  │  │ [✎] [🗑]  │  │ [✎] [🗑]  │              │
│   └───────────┘  └───────────┘  └───────────┘              │
│                                                             │
│   (Empty state: "Create your first board" with big button) │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Board Cards
- 200x160px cards in a responsive grid
- Thumbnail preview (solid color initially, screenshot later)
- Board name (editable on click or via edit button)
- Last modified date
- Edit (rename) and Delete buttons
- Hover effect with subtle highlight

### Interactions
- **Click card** → Open board
- **+ New Board** → Create board with default name "Untitled Board"
- **Edit button** → Inline rename with text input
- **Delete button** → Confirmation dialog, then delete

---

## Implementation Steps

### Phase 1: Data Layer (~3 files)
1. Create `src/board_index.rs` - BoardMetadata, BoardIndex structs with CRUD methods
2. Modify `src/board.rs` - Add board_id field, update save/load to use ID-based paths
3. Add migration logic - Convert existing `board.json` to new format on first run

### Phase 2: App State (~2 files)
4. Create `src/landing.rs` - Landing page component and rendering
5. Modify `src/app.rs` - Add AppView enum, board_index field, view switching methods

### Phase 3: Landing UI (~1 file)
6. Implement landing page render in `src/landing.rs`:
   - Header bar with title and "New Board" button
   - Board card grid
   - Empty state
   - Edit/delete interactions

### Phase 4: Navigation & Integration (~3 files)
7. Modify `src/main.rs` - Start with Landing view
8. Modify `src/render.rs` - Route rendering based on AppView
9. Add navigation actions - GoHome, OpenBoard(id), CreateBoard, RenameBoard, DeleteBoard

### Phase 5: Polish
10. Add confirmation dialog for delete
11. Keyboard shortcuts (Cmd+N for new board, Escape to go home)
12. Smooth transitions between views

---

## New Files
- `src/board_index.rs` - Board metadata and index management
- `src/landing.rs` - Landing page UI

## Modified Files
- `src/app.rs` - AppView state, view switching
- `src/board.rs` - ID-based storage, migration
- `src/render.rs` - View routing
- `src/main.rs` - Initial view, new keybindings
- `src/actions.rs` - New actions for board CRUD

---

## Migration Strategy
On app startup:
1. Check if `index.json` exists
2. If not, check for legacy `board.json`
3. If legacy exists, migrate: create index, move board to `boards/{new-uuid}.json`
4. If nothing exists, start fresh with empty index

---

## Key Decisions Made
1. **UUID-based board IDs** - Avoids naming conflicts, allows renames
2. **Separate index file** - Fast loading of board list without parsing all boards
3. **AppView enum** - Clean state machine for navigation
4. **Optional Board** - Board only loaded when viewing, saves memory
5. **No thumbnails initially** - Can add screenshot-based previews later

---

## Estimated Scope
- ~400-500 new lines of code
- 2 new files, 5 modified files
- Core functionality without thumbnails

---

## Questions for User
None - the scope is clear. Ready to implement.
