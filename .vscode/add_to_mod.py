#!/usr/bin/env python3
"""
Script to automatically add a Rust file to its parent mod.rs or main.rs
Usage: python add_to_mod.py <filepath>
"""

import sys
from pathlib import Path

def snake_to_module_name(filename):
    """Convert filename to module name (remove .rs extension)"""
    return filename.replace('.rs', '')

def add_to_mod_file(file_path):
    """Add the given file to its parent mod.rs"""
    file_path = Path(file_path).resolve()
    
    if not file_path.exists() or not file_path.name.endswith('.rs'):
        print(f"Error: {file_path} is not a valid Rust file")
        return False
    
    # If it's main.rs or lib.rs, don't add it
    if file_path.name in ['main.rs', 'lib.rs']:
        print(f"Skipping {file_path.name} - no need to add to mod")
        return False
    
    # For mod.rs files, we need to add them to the parent scope
    is_mod_file = file_path.name == 'mod.rs'
    
    module_name = snake_to_module_name(file_path.name) if not is_mod_file else file_path.parent.name
    
    if is_mod_file:
        # For mod.rs, look in the parent directory
        parent_dir = file_path.parent.parent
        target_dir_name = file_path.parent.name
    else:
        parent_dir = file_path.parent
        target_dir_name = None
    
    
    # Check for mod.rs in the parent directory
    mod_file = parent_dir / 'mod.rs'
    
    if not mod_file.exists():
        # If no mod.rs, check if we're in or need to look at src/ directory
        if parent_dir.name == 'src':
            main_file = parent_dir / 'main.rs'
            lib_file = parent_dir / 'lib.rs'
            
            if main_file.exists():
                mod_file = main_file
            elif lib_file.exists():
                mod_file = lib_file
            else:
                print(f"Error: No mod.rs, main.rs, or lib.rs found")
                return False
        else:
            print(f"Error: No mod.rs found in {parent_dir}")
            return False
    
    # Read the current content
    with open(mod_file, 'r') as f:
        content = f.read()
    
    mod_declaration = f"mod {module_name};"
    pub_use_statement = f"pub use {module_name}::*;"
    
    # Check if module is already declared
    if f"mod {module_name};" in content:
        print(f"✓ Module '{module_name}' already declared in {mod_file.name}")
        return True
    
    # Find where to insert the mod declaration
    lines = content.split('\n')
    insert_index = 0
    
    # Find the last mod declaration
    for i, line in enumerate(lines):
        if line.strip().startswith('mod '):
            insert_index = i + 1
    
    # If no mod declarations found, insert at the top
    if insert_index == 0:
        # Skip initial comments and empty lines
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('//') and not line.strip().startswith('/*'):
                insert_index = i
                break
    
    # Insert the mod declaration
    lines.insert(insert_index, mod_declaration)
    
    # Write back
    with open(mod_file, 'w') as f:
        f.write('\n'.join(lines))
    
    print(f"✓ Added 'mod {module_name};' to {mod_file.name}")
    return True

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python add_to_mod.py <filepath>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    success = add_to_mod_file(file_path)
    sys.exit(0 if success else 1)
