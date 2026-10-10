# `aril` (அரில்)

<div align="center">

<img src="https://digital-tamizh.web.app/Aril/Aril_OgBanner.jpg" alt="ARIL Tamil Unicode Engine: 3D isometric diorama depicting chaotic legacy 8-bit font glyphs transforming through a glowing quantum gateway into illuminated Tamil Unicode blocks spelling 'முருகன்' across a digital bridge" width="850px" style="border-radius: 12px; box-shadow: 0 8px 24px rgba(0,0,0,0.25);" />

### High-performance Tamil legacy font encoding to Unicode converter in Rust.
#### Multi-threaded CLI & lightweight WebAssembly engine.

> *“...அதங்கோட்டு ஆசாற்கு **அரில் தபத் தெரிந்து**  
> மயங்கா மரபின் எழுத்துமுறை காட்டி...”*  
> — **தொல்காப்பியம் (பாயிரப் பகுதி)**

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/digital-tamil/aril)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![HuggingFace Dataset](https://img.shields.io/badge/🤗_Corpus-Tamil_Heritage-yellow)](https://huggingface.co/datasets/Sanjaiyan/Tamil-Digital-Heritage-Corpus)
[![Engine](https://img.shields.io/badge/Automaton-Aho--Corasick-blue)](#-the-algorithmic-quantum-leap)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue.svg)](#-license)
[![Status](https://img.shields.io/badge/Status-Active%20Development-success)]()

[Philosophy](#-the-philosophy--origin-story) •
[Architecture](#-workspace-architecture) •
[Benchmarks & Engine](#-the-algorithmic-quantum-leap) •
[Supported Fonts](#-supported-encodings-29) •
[CLI Quickstart](#-cli-usage-guide) •
[Library Usage](#-rust-library-usage) •
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

###  The `aril` Approach: Single-Pass Aho-Corasick Automata
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
│ In complex Grantha / compound glyphs:                                                 │
│                                                                                        │
│   "û"   => "க்ஷ"   (Base consonant)                                                    │
│   "û‘"  => "க்ஷா"  (Base + Long AA sign)                                              │
│   "û’"  => "க்ஷி"  (Base + Short I sign)                                              │
│                                                                                        │
│ Under naive search-and-replace, matching "û" first prematurely consumes the prefix,    │
│ leaving an orphaned trailing modifier "‘" or "’" and corrupting the output.            │
│ aril's compiled automaton deterministically resolves the longest valid sequence       │
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
│   ├── aril-cli/         ✅ High-speed multi-threaded CLI, ANSI REPL & diff viewer
│   └── aril-wasm/        🚧 WebAssembly bindings & JS API (Work In Progress)
├── Cargo.toml
└── README.md
```

| Crate | Status | Description |
|---|:---:|---|
| **`aril-core`** | **Done** | The pure Rust library. Houses all 29+ static font tables, the Aho-Corasick automaton compiler, heuristic encoding detection, and Rayon parallel batch handlers. |
| **`aril-cli`** | **Done** | Command-line utility. Features batch folder recursion, atomic file writing with `.bak` safety guards, stdin/stdout streaming, shell completions, and an ANSI interactive REPL. |
| **`aril-wasm`** | **WIP** | Browser & Node.js WASM target. Exposes streaming zero-copy converters to power client-side web applications like [tamil-simple-ocr](https://digital-tamizh.web.app/tamil-simple-ocr/). |

---

## 🗂️ Supported Encodings (29+)

`aril` includes bidirectional mapping matrices compiled directly into static binary memory:

### 1. DTP & Publishing Fonts
* **Bamini** (`bamini` / `baamini`) — The ubiquitous standard across Sri Lankan & Tamil diaspora publishing.
* **ShreeLipi** (`shreelipi` / `shree-tamil`) — Industry benchmark for books, advertising, and magazines.
* **Softview** (`softview`) — Widespread desktop publishing standard across South India.
* **Amudham** (`amudham`) — Classical typography & regional literary press encoding.
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

### Installation (Local Build)

```bash
# Clone the repository
git clone https://github.com/digital-tamil/aril.git
cd aril

# Build release binaries
cargo build --release

# Optional: Link aril to your local path
cargo install --path crates/aril-cli
```

---

### Basic Conversion

#### Pipe text via `stdin`:
```bash
echo "jkpHf«" | aril convert --from bamini
# Output: தமிழகம்
```

#### Convert a file:
```bash
aril convert --from bamini input_legacy.txt -o output_unicode.txt
```

---

### Heuristic Encoding Auto-Detection

Not sure which font an archival document was written in? `aril` inspects byte patterns and characteristic ligature signatures:

```bash
aril detect sample_legacy.txt
```

```text
[✓] Detected Encoding: Bamini (Confidence: 98.4%)
[i] Signature Match: Characteristic vowel split markers [0xA1, 0xED]
```

Or convert directly with automatic detection:
```bash
aril convert --from auto archive.txt -o converted.txt
```

---

### Interactive Terminal REPL & Diff Preview

Test encodings interactively with live terminal output and side-by-side ANSI diff highlighting:

```bash
aril repl --encoding bamini
```

```
┌───────────────────────────────────────────────────────┐
│                 ARIL INTERACTIVE REPL                 │
│               Encoding: Bamini ➔ Unicode              │
│       Type text to inspect. Press Ctrl+C to exit.     │
└───────────────────────────────────────────────────────┘
>> tz¡f«
── Legacy  : tz¡f«
── Unicode : வணக்கம்
```

---

### Corpus-Scale Batch Processing & Safety

When processing millions of files for dataset creation, `aril` includes enterprise safety guarantees:

```bash
# Recursively convert an entire archival directory
aril convert \
  --from bamini \
  --batch "legacy_books/**/*.txt" \
  --in-place \
  --backup \
  --threads 16
```

- **Atomic Writes:** Prevents file corruption if an external interruption occurs mid-write.
- **Automatic `.bak` creation:** Safely archives the source file before modifying contents.
- **Binary Filtering:** Skips image binaries, compiled objects, and invalid UTF-8 automatically.

---

## 🦀 Rust Library Usage (`aril-core`)

Add `aril-core` to your `Cargo.toml`:

```toml
[dependencies]
aril-core = { git = "https://github.com/digital-tamil/aril", package = "aril-core" }
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
    let auto_converted = auto_to_unicode(mysterious_legacy)?;
    println!("Auto-converted: {}", auto_converted); // "யாதும் ஊரே யாவரும் கேளிர்"

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

## 📦 Distribution & Roadmap

- [x] **`aril-core`**: High-performance Aho-Corasick engine & 29+ font mapping matrices.
- [x] **`aril-cli`**: Multi-threaded CLI, batch safety, interactive ANSI REPL, and diff viewer.
- [ ] **`aril-wasm` (In Progress)**: Browser and Node.js WebAssembly target for instant client-side OCR post-processing.
- [ ] **Package Registry Releases (Coming Soon)**:
  - 🦀 **crates.io**: `cargo add aril-core`
  - 🌐 **npm**: `@digital-tamil/aril` (zero-overhead WASM bridge)
  - 🐍 **PyPI**: `pip install aril` (High-performance PyO3 Python bindings for AI/ML pipelines)

---

## 🤝 Lineage & Acknowledgments

`aril` honors the trailblazers of digital Tamil computing:

- **[open-tamil](https://github.com/Ezhil-Language-Foundation/open-tamil)** (`tamil/txt2unicode`): Grateful acknowledgment to the Open-Tamil authors and maintainers. Their pioneering catalog of legacy font glyph pair tables (such as mapping tuples like `("j", "த")`, `("kp", "மி")`, `("o;", "ழ்")`) served as the original inspiration and lexical ground truth for constructing `aril`'s state machines.
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
