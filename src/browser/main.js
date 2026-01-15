// Import tree-sitter - it's loaded as a global
const Parser = TreeSitter;
import init, { process_chord_progression } from './pkg/chord_processor.js';

let parser = null;
let wasmReady = false;

/**
 * Convert Tree-sitter CST Node to JSON AST
 */
function nodeToJson(node) {
    const kind = node.type;
    
    if (kind === 'progression') {
        const children = [];
        for (let i = 0; i < node.childCount; i++) {
            const child = node.child(i);
            if (child.type === 'note') {
                children.push(nodeToJson(child));
            }
        }
        return {
            type: 'progression',
            children: children
        };
    } else if (kind === 'note') {
        return {
            type: 'note',
            text: node.text
        };
    }
    
    return null;
}

/**
 * Initialize Tree-sitter and WASM
 */
async function initialize() {
    try {
        // Initialize web-tree-sitter
        await Parser.init();
        parser = new Parser();
        const Lang = await Parser.Language.load('tree-sitter-chordprog.wasm');
        parser.setLanguage(Lang);
        
        // Initialize Rust WASM
        await init();
        wasmReady = true;
        
        console.log('Initialized successfully');
        return true;
    } catch (error) {
        console.error('Initialization failed:', error);
        return false;
    }
}

/**
 * Parse and process chord progression
 */
function parseChordProgression(input) {
    if (!parser || !wasmReady) {
        throw new Error('Parser not initialized');
    }
    
    // Parse with Tree-sitter
    const tree = parser.parse(input);
    const root = tree.rootNode;
    
    // Convert CST to JSON AST
    const jsonAst = nodeToJson(root);
    
    // Process with Rust WASM
    const result = process_chord_progression(JSON.stringify(jsonAst));
    
    // Check for errors in the result
    const parsed = JSON.parse(result);
    if (parsed && typeof parsed === 'object' && 'error' in parsed) {
        throw new Error(parsed.error || 'Chord progression processing error');
    }
    
    return parsed;
}

/**
 * Setup UI event handlers
 */
function setupUI() {
    const inputEl = document.getElementById('input');
    const parseBtn = document.getElementById('parseBtn');
    const outputEl = document.getElementById('output');
    
    parseBtn.addEventListener('click', () => {
        const input = inputEl.value.trim();
        
        if (!input) {
            outputEl.textContent = 'Please enter a chord progression';
            outputEl.className = 'error';
            return;
        }
        
        try {
            const result = parseChordProgression(input);
            console.log(result);
            outputEl.innerHTML = '<span class="success"></span>';
            const successSpan = outputEl.querySelector('.success');
            if (successSpan) {
                successSpan.textContent = `Result: ${JSON.stringify(result)}`;
            }
        } catch (error) {
            outputEl.innerHTML = '<span class="error"></span>';
            const errorSpan = outputEl.querySelector('.error');
            if (errorSpan) {
                errorSpan.textContent = `Error: ${error.message}`;
            }
            console.error(error);
        }
    });
}

/**
 * Main entry point
 */
async function main() {
    const outputEl = document.getElementById('output');
    outputEl.innerHTML = 'Loading...';
    
    const success = await initialize();
    
    if (success) {
        outputEl.innerHTML = 'Ready! Enter a chord progression and click Parse.';
        setupUI();
    } else {
        outputEl.innerHTML = '<span class="error">Failed to initialize. Check console for details.</span>';
    }
}

main();
