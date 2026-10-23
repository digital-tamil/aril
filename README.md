# `aril` (அரில்)

<div align="center">

<img src="https://digital-tamizh.web.app/Aril/Aril_OgBanner.jpg" alt="ARIL Tamil Unicode Engine: 3D isometric diorama depicting chaotic legacy 8-bit font glyphs transforming through a glowing quantum gateway into illuminated Tamil Unicode blocks spelling 'முருகன்' across a digital bridge" width="850px" style="border-radius: 12px; box-shadow: 0 8px 24px rgba(0,0,0,0.25);" />

### High-performance Tamil legacy font encoding to Unicode converter in Rust.
#### Multi-threaded CLI with native PDF extraction & lightweight WebAssembly engine.

> *“...அதங்கோட்டு ஆசாற்கு **அரில் தபத் தெரிந்து**  
> மயங்கா மரபின் எழுத்துமுறை காட்டி...”*  
> — **தொல்காப்பியம் (பாயிரப் பகுதி)**

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/digital-tamil/aril)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![HuggingFace Dataset](https://img.shields.io/badge/🤗_Corpus-Tamil_Heritage-yellow)](https://huggingface.co/datasets/Sanjaiyan/Tamil-Digital-Heritage-Corpus)
[![crates.io](https://img.shields.io/crates/v/aril-core.svg?color=fc8d62&logo=rust)](https://crates.io/crates/aril-core)
[![npm (CLI)](https://img.shields.io/npm/v/@digital-tamil/aril.svg?color=cb3837&logo=npm&label=npm%20cli)](https://www.npmjs.com/package/@digital-tamil/aril)
[![npm (WASM)](https://img.shields.io/npm/v/@digital-tamil/aril-wasm.svg?color=cb3837&logo=webassembly&label=npm%20wasm)](https://www.npmjs.com/package/@digital-tamil/aril-wasm)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue.svg)](#-license)

[Philosophy](#-the-philosophy--origin-story) •
[Architecture](#-workspace-architecture) •
[Benchmarks & Engine](#-the-algorithmic-quantum-leap) •
[Supported Fonts](#-supported-encodings-29) •
[CLI Quickstart](#-cli-usage-guide) •
[Rust Library](#-rust-library-usage) •
[WebAssembly](#-webassembly-usage-digital-tamilaril-wasm) •
[Roadmap](#-distribution--roadmap) •
[Credits & Lineage](#-lineage--acknowledgments)

</div>

---

## 📜 The Philosophy & Origin Story

In the special prologue (*சிறப்புப்பாயிரம்*) of **தொல்காப்பியம்** (Tholkappiyam)—the oldest extant grammar and linguistic treatise of the Tamil language—the poet Panambaranar describes how Tholkappiyar presented his magnum opus before the council of the Pandyan king:

$$\text{“அதங்கோட்டு ஆசாற்கு அரில் தபத் தெரிந்து...”}$$

In classical Tamil, **அரில் (Aril)** means *knot, tangled cluster, brier, or confusion*. **அரில் தபத் தெரிந்து** signifies *untangling all knots, resolving obscurities, and laying bare the pure, unadulterated truth*.

### The Dark Age of Digital Tamil Typesetting
Between 1985 and the late 2000s, millions of historical books, government gazettes, news archives, and literary classics were typeset using proprietary 8-bit Latin/ANSI font glyph overlays (**Bamini, ShreeLipi, Softview, Dinakaran, Murasu, TAB, TAM**). The raw text inside these digital archives is not Tamil—it is a mangled, unsearchable bramble of English characters and high-ASCII codes:

$$\text{“KUfd;”} \xrightarrow{\text{Disentangle}} \text{“முருகன்”}$$

### The Catalyst: OCR & The Digital Heritage Corpus
When building the browser-grade **[Tamil Simple OCR](https://digital-tamizh.web.app/tamil-simple-ocr/)** and curating the multi-million-token **[Tamil Digital Heritage Corpus](https://huggingface.co/datasets/Sanjaiyan/Tamil-Digital-Heritage-Corpus)** on Hugging Face, existing legacy converters buckled. They relied on sequential string replacements in interpreted scripting languages, taking days to process terabytes of digitized heritage text while producing silent ligature corruption.

**`aril` was engineered to be the definitive solution:** a zero-allocation, deterministic, single-pass state-machine engine that clears away decades of font fragmentation at memory-bus speeds.

---

## ⚡ The Algorithmic Quantum Leap

### ❌ The Legacy Approach: Sequential Regex Cascades
Traditional converters (including legacy Python tools) execute sequentially looped string replacements or cascading regex pipelines:

```
Input String ──> Rule 1 (Replace) ──> Rule 2 (Replace) ──> ... ──> Rule 300 ──> Output
Complexity: O(M × N) allocations | Continuous heap thrashing & substring scanning
```

If a font mapping dictionary has $M = 300$ glyph rules and the input text has $N$ characters, the converter scans the string $M$ times, repeatedly allocating intermediate heap memory. Crucially, **rule-order sensitivity** often causes greedy prefix collisions (e.g., modifying a base letter before its multi-glyph vowel ligature has matched).

### 🚀 The `aril` Approach: Single-Pass Aho-Corasick Automata
`aril` compiles static bidirectional mapping tables into deterministic finite state machines using the **Aho-Corasick** algorithm configured with **`LeftmostLongest`** match semantics:

```
                           ┌────────────────────────┐
                           │   Forward Automaton    │
Legacy 8-bit Stream ─────> │ (Aho-Corasick Matcher) │ ─────> Pure UTF-8 Unicode
                           │ MatchKind::Longest     │
                           └────────────────────────┘
Complexity: O(N) single-pass linear time | Zero backtracking | Pre-sized buffer
```

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Why LeftmostLongest Matters for Indic / Tamil Ligatures                                │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ In complex Grantha / compound glyphs:                                                  │
│                                                                                        │
│   "û"   => "க்ஷ"   (Base consonant)                                                     │
│   "û‘"  => "க்ஷா"  (Base + Long AA sign)                                                │
│   "û’"  => "க்ஷி"  (Base + Short I sign)                                                 │
│                                                                                        │
│ Under naive search-and-replace, matching "û" first prematurely consumes the prefix,    │
│ leaving an orphaned trailing modifier "‘" or "’" and corrupting the output.            │
│ aril's compiled automaton deterministically resolves the longest valid sequence        │
│ in a single pass without backtracking overhead.                                        │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Systems Engineering Highlights
- **Pre-sized Buffers:** Forward conversion pre-allocates `input.len() * 3` bytes to seamlessly accommodate the 1-byte legacy ASCII to 3-byte UTF-8 Tamil expansion without intermediate reallocations.
- **Lock-Free Zero-Cost Caching:** Compiled automata are held in a static array guarded by `std::sync::OnceLock`. Converter lookups by `Encoding` discriminant run in lock-free $O(1)$ time.
- **Rayon Multi-Core Scaling:** Slices of text collections are converted over data-parallel threads using `convert_batch_parallel`, maxing out NVMe I/O throughput.

---

## 🏛️ Workspace Architecture

The repository is structured as an ergonomic Rust Cargo workspace:

```
aril/
├── crates/
│   ├── aril-core/        ✅ Core conversion engine, mapping tables & heuristic detector
│   ├── aril-cli/         ✅ High-speed multi-threaded CLI with native PDF extraction
│   └── aril-wasm/        ✅ WebAssembly bindings & client-side PDF pipeline
├── Cargo.toml
└── README.md
```

| Crate | Status | Description |
|---|:---:|---|
| **`aril-core`** | **Done** | Pure Rust library. Houses all 29+ static font tables, the Aho-Corasick automaton compiler, heuristic encoding detection, and Rayon parallel batch handlers. |
| **`aril-cli`** | **Done** | Command-line utility with native PDF text extraction, batch folder recursion, atomic file writing with `.bak` safety, stdin/stdout streaming, and an interactive ANSI REPL. |
| **`aril-wasm`** | **Done** | High-performance WebAssembly engine with client-side PDF text extraction and font conversion for browsers and Node.js (`@digital-tamil/aril-wasm`). |

---

## 🗂️ Supported Encodings (29+)

`aril` includes bidirectional mapping matrices compiled directly into static binary memory:

### 1. DTP & Publishing Fonts
* **Bamini** (`bamini` / `baamini`) — The ubiquitous standard across Sri Lankan & Tamil diaspora publishing.
* **ShreeLipi** (`shreelipi` / `shree-tamil`) — Industry benchmark for books, advertising, and magazines.
* **Softview** (`softview`) — Widespread desktop publishing standard across South India.
* **Vanavil** (`vanavil`) — Foundational DTP standard across academic and literary presses.
* **Amudham** (`amudham`) — Classical typography & regional press encoding.
* **TAB** (`tab`) & **TAM** (`tam`) — Tamil script Monolingual and Bilingual standards.

### 2. Newspapers & Media Presses
* **Dinakaran** (`dinakaran`) — Daily news layout formatting.
* **Thinakaran** (`thinakaran`) — Sri Lankan national daily newspaper press standard.
* **Dina Thanthi** (`dailythanthi` / `dinathanthi`) — High-circulation daily font table.
* **Vikatan** (`vikatan`) — Pioneering weekly magazine publishing encoding.
* **Webtamizh** (`webtamizh`) — Early digital journalism encoding.

### 3. Standardization & Transliteration
* **TSCII** (`tscii`) — Tamil Standard Code for Information Interchange (RFC standard).
* **Anjal** (`anjal`) — Phonetic transliteration input system encoding.
* **Murasu** (`murasu`) — Early digital Tamil computing font mapping.
* *...and over a dozen regional custom typestyles.*

---

## 💻 CLI Usage Guide (`aril-cli`)

### Installation

#### One-Line Shell Installer (Linux & macOS):
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/digital-tamil/aril/releases/latest/download/aril-cli-installer.sh | sh
```

#### PowerShell (Windows):
```powershell
powershell -c "irm https://github.com/digital-tamil/aril/releases/latest/download/aril-cli-installer.ps1 | iex"
```

#### Via npm (Cross-Platform CLI):
```bash
npm install -g @digital-tamil/aril

# Or run instantly on-demand without installing:
npx @digital-tamil/aril --help
```

#### Via Cargo:
```bash
# Fast precompiled binary
cargo binstall aril-cli

# Or compile from source
cargo install aril-cli
```

---

### Basic Conversion

#### Pipe text via `stdin`:
```bash
echo "jkpoPok;" | aril --from bamini
# Output: தமிழீழம்
```

#### Convert inline text directly:
```bash
aril -s "jkpoPok;" -f bamini
# Output: தமிழீழம்
```

#### Convert a file:
```bash
aril -f bamini input_legacy.txt -o output_unicode.txt
```

---

### 📑 PDF Text Extraction & Legacy Font Conversion

Extract and decode legacy Tamil text directly from PDF documents with custom page range syntax:

```bash
# Convert entire PDF to Unicode
aril heritage_book.pdf -o converted.txt

# Convert selective pages with visible page markers ("--- பக்கம் N ---")
aril sangam_manuscript.pdf --pages "1-5,8,18-" --page-markers -o output.txt

# Batch convert a directory of legacy PDFs into a dedicated output folder
aril "archives/**/*.pdf" -r --out-dir ./converted_texts/
```

---

### Heuristic Encoding Auto-Detection

Inspect byte patterns and characteristic ligature signatures without guessing:

```bash
aril detect sample_legacy.txt
```

```text
sample_legacy.txt: Bamini
```

Or convert directly with automatic detection (default behavior):
```bash
aril archive.txt -o converted.txt
```

---

### Interactive Terminal REPL & Diff Preview

Test encodings interactively with live terminal output and side-by-side ANSI diff highlighting:

```bash
aril repl
```

```text
┌───────────────────────────────────────────────────────┐
│                 ARIL INTERACTIVE REPL                 │
│               Encoding: Bamini ➔ Unicode              │
│       Type text to inspect. Press Ctrl+C to exit.     │
└───────────────────────────────────────────────────────┘
>> mud; mup mad; mz;lu;f;Fk; mupahs; cik mUs; KUNfhNd
── Legacy  : mud; mup mad; mz;lu;f;Fk; mupahs; cik mUs; KUNfhNd
── Unicode : அரன் அரி அயன் அண்டர்க்கும் அரியாள் உமை அருள் முருகோனே
```

---

### Corpus-Scale Batch Processing & Safety

When processing millions of files for dataset creation, `aril` includes enterprise safety guarantees:

```bash
# Recursively convert an entire archival directory in-place with backups
aril "legacy_books/**/*.txt" \
  --from bamini \
  -r \
  --in-place \
  --backup
```

- **Atomic Writes:** Prevents file corruption if an external interruption occurs mid-write.
- **Automatic `.bak` creation:** Safely archives the source file before modifying contents.
- **Binary Filtering:** Skips image binaries, compiled objects, and invalid UTF-8 automatically.

---

## 🦀 Rust Library Usage (`aril-core`)

Add `aril-core` to your `Cargo.toml`:

```bash
cargo add aril-core
```

Or configure manually in `Cargo.toml`:

```toml
[dependencies]
aril-core = "0.1.0"
```

### Deterministic Text Transformation

```rust
use aril_core::{to_unicode, auto_to_unicode, Encoding};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Explicit encoding conversion (O(N) single-pass)
    let legacy_text = "jkpoPok;";
    let unicode = to_unicode(legacy_text, Encoding::Bamini)?;
    println!("Unicode: {}", unicode); // "தமிழீழம்"

    // 2. Autonomous heuristic conversion
    let mysterious_legacy = "Â¡Ðõ °§Ã Â¡ÅÕõ §¸Ç¢÷";
    let (auto_converted, detected) = auto_to_unicode(mysterious_legacy)?;
    println!("Detected: {:?}, Text: {}", detected, auto_converted); // "யாதும் ஊரே யாவரும் கேளிர்"

    Ok(())
}
```

### High-Throughput Rayon Batch Conversion

```rust
#[cfg(feature = "parallel")]
use aril_core::{get_converter, Encoding};

fn process_corpus(documents: &[String]) {
    let converter = get_converter(Encoding::Bamini);
    
    // Converts in parallel across all CPU cores with zero lock contention
    let results: Vec<String> = converter.convert_batch_parallel(documents);
    println!("Successfully processed {} heritage texts.", results.len());
}
```

---

## 🌐 WebAssembly Usage (`@digital-tamil/aril-wasm`)

Install the browser and Node.js package:

```bash
npm install @digital-tamil/aril-wasm
```

### 1. In Browser / Web Apps (Vite, Next.js, React)

```javascript
import init, { auto_to_unicode, to_unicode, convert_pdf_to_unicode } from "@digital-tamil/aril-wasm";

// Initialize WASM binary once
await init();

// Auto-detect and convert legacy text (returns { text, encoding })
const { text, encoding } = auto_to_unicode("ePykzp kplw;W xUtd;");
console.log(text);     // "நீலமணி மிடற்று ஒருவன்"
console.log(encoding); // "Bamini"

// Explicit conversion (Bamini -> Unicode)
const unicode = to_unicode("khNahd;", "bamini");
console.log(unicode);  // "மாயோன்"

// Client-side PDF text extraction & font conversion from an <input type="file">
const file = document.getElementById("pdfInput").files[0];
const arrayBuffer = await file.arrayBuffer();
const pdfText = convert_pdf_to_unicode(new Uint8Array(arrayBuffer), "bamini");
console.log(pdfText);
```

### 2. In Node.js / Server Runtimes

```javascript
import fs from "node:fs/promises";
import init, { auto_to_unicode, convert_pdf } from "@digital-tamil/aril-wasm";

await init();

// Read PDF binary and extract structured Tamil document data
const pdfBuffer = await fs.readFile("./tolkāppiyam.pdf");
const doc = convert_pdf(new Uint8Array(pdfBuffer), {
  encoding: "auto",
  startPage: 1,
  endPage: 5,
  includePageMarkers: true,
});

console.log(`Total Pages: ${doc.totalPages}`);
console.log(`Detected Fonts: ${doc.detectedFonts.join(", ")}`);
console.log(doc.combinedText);
```

---

## 📦 Distribution & Roadmap

- [x] **`aril-core`**: High-performance Aho-Corasick engine & 29+ font mapping matrices.
- [x] **`aril-cli`**: Multi-threaded CLI with native PDF extraction, batch safety, interactive REPL.
- [x] **`aril-wasm`**: Client-side WebAssembly bindings with PDF support for browsers and Node.js.
- [x] **Package Registries**:
  - 🦀 **crates.io**: `cargo add aril-core` & `cargo install aril-cli`
  - 🌐 **npm (CLI)**: `npm install -g @digital-tamil/aril`
  - 🌐 **npm (WASM)**: `npm install @digital-tamil/aril-wasm`
  - ⚡ **Standalone Binaries**: Native shell & PowerShell zero-dependency installers.
- [ ] **PyPI (Next Milestone)**: `pip install aril` (PyO3 Python bindings for AI/ML data pipelines).

---

## 🤝 Lineage & Acknowledgments

`aril` honors the trailblazers of digital Tamil computing:

- **[open-tamil](https://github.com/Ezhil-Language-Foundation/open-tamil/tree/main/tamil/txt2unicode)** (`tamil/txt2unicode`): Grateful acknowledgment to the Open-Tamil authors and maintainers. Their pioneering catalog of legacy font glyph pair tables (such as mapping tuples like `("j", "த")`, `("kp", "மி")`, `("o;", "ழ்")`) served as the original inspiration and lexical ground truth for constructing `aril`'s state machines.
- **[Tamil Digital Heritage Corpus](https://huggingface.co/datasets/Sanjaiyan/Tamil-Digital-Heritage-Corpus)**: The dataset preservation effort driving the need for industrial-throughput legacy conversion.
- **[Tamil Simple OCR](https://digital-tamizh.web.app/tamil-simple-ocr/)**: The browser-native OCR engine whose legacy font decoding pipeline relies on `aril`.

---

## 📄 License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

<div align="center">

Crafted with reverence for Tamil literature and devotion to systems engineering.  
**Made by [Digital Tamil](https://github.com/digital-tamil)**

</div>
