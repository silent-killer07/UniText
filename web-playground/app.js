import init, { UniText } from '../bindings/wasm/pkg/unitext_wasm.js';

let wasmLoaded = false;

// DOM Elements
const textInput = document.getElementById('text-input');
const tabBtns = document.querySelectorAll('.tab-btn');
const tabContents = document.querySelectorAll('.tab-content');

// Analyze
const analyzeStats = document.getElementById('analyze-stats');
const graphemeTable = document.querySelector('#grapheme-table tbody');

// Security
const securityBadge = document.getElementById('security-badge');
const securityDetails = document.getElementById('security-details');

// Convert
const asciiOutput = document.getElementById('ascii-output');
const asciiLossy = document.getElementById('ascii-lossy');

// Compare
const comp1 = document.getElementById('compare-1');
const comp2 = document.getElementById('compare-2');
const compResults = document.getElementById('compare-results');

async function initializeApp() {
    try {
        await init();
        wasmLoaded = true;
        console.log("UniText WASM initialized!");
        
        // Initial process
        if (textInput.value) {
            processText();
        } else {
            textInput.value = "Héllo 👨‍👩‍👧‍👦 Café";
            processText();
        }
        processCompare();
    } catch (e) {
        console.error("Failed to load WASM module", e);
        textInput.value = "Error loading WebAssembly module. Did you run `wasm-pack build`?";
    }
}

function processText() {
    if (!wasmLoaded) return;
    const text = textInput.value;
    if (!text) return;

    try {
        // 1. Analyze
        const analysisStr = UniText.analyze(text);
        const analysis = JSON.parse(analysisStr);
        
        analyzeStats.innerHTML = `
            <div class="stat-card">
                <div class="stat-label">Graphemes</div>
                <div class="stat-value">${analysis.graphemes_count}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">Code Points</div>
                <div class="stat-value">${analysis.code_points_count}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">Bytes (UTF-8)</div>
                <div class="stat-value">${analysis.bytes_count}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">Dominant Script</div>
                <div class="stat-value">${analysis.script}</div>
            </div>
        `;

        graphemeTable.innerHTML = analysis.grapheme_breakdown.map(g => `
            <tr>
                <td>${g.slot}</td>
                <td style="font-size: 1.2rem">${g.char}</td>
                <td>${g.script}</td>
                <td>${g.category}</td>
            </tr>
        `).join('');

        // 2. Security
        const secStr = UniText.is_safe(text);
        const sec = JSON.parse(secStr);
        
        securityBadge.className = 'risk-badge';
        if (sec.risk_score === 0) {
            securityBadge.classList.add('risk-safe');
            securityBadge.textContent = 'SAFE ✅';
        } else if (sec.risk_score < 50) {
            securityBadge.classList.add('risk-low');
            securityBadge.textContent = 'LOW RISK ⚠️';
        } else {
            securityBadge.classList.add('risk-high');
            securityBadge.textContent = 'HIGH RISK 🔴';
        }
        
        securityDetails.innerHTML = `
            <p><strong>Diagnosis:</strong> ${sec.details}</p>
            <p><strong>Mixed Script:</strong> ${analysis.is_mixed_script ? '<span class="compare-no">Yes</span>' : '<span class="compare-yes">No</span>'}</p>
        `;

        // 3. Convert
        const convStr = UniText.to_ascii(text);
        const conv = JSON.parse(convStr);
        
        asciiOutput.textContent = conv.output;
        if (conv.lossy) {
            asciiLossy.className = 'lossy-indicator lossy-yes';
            asciiLossy.innerHTML = '⚠️ Lossy Conversion (Transliterated)';
        } else {
            asciiLossy.className = 'lossy-indicator lossy-no';
            asciiLossy.innerHTML = '✅ Lossless Conversion';
        }

    } catch (e) {
        console.error("Processing error", e);
    }
}

function processCompare() {
    if (!wasmLoaded) return;
    const t1 = comp1.value;
    const t2 = comp2.value;
    
    if (!t1 || !t2) return;
    
    try {
        const isVisuallyEqual = UniText.visually_equal(t1, t2);
        const isByteEqual = t1 === t2;
        
        compResults.innerHTML = `
            <div class="compare-result-item">
                <span>Byte-level Equality (UTF-8)</span>
                <span class="${isByteEqual ? 'compare-yes' : 'compare-no'}">${isByteEqual ? 'Yes ✅' : 'No ❌'}</span>
            </div>
            <div class="compare-result-item">
                <span>Visual Equality (Homograph Check)</span>
                <span class="${isVisuallyEqual ? 'compare-yes' : 'compare-no'}">${isVisuallyEqual ? 'Yes ✅' : 'No ❌'}</span>
            </div>
        `;
    } catch (e) {
        console.error(e);
    }
}

// Event Listeners
textInput.addEventListener('input', processText);
comp1.addEventListener('input', processCompare);
comp2.addEventListener('input', processCompare);

tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        tabBtns.forEach(b => b.classList.remove('active'));
        tabContents.forEach(c => c.classList.remove('active'));
        
        btn.classList.add('active');
        document.getElementById(`tab-${btn.dataset.tab}`).classList.add('active');
    });
});

// Start
initializeApp();
