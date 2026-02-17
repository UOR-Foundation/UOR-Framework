//! Generates the JSON search index and search.js client.

use anyhow::Result;
use serde_json;

use crate::extractor::build_search_index;

/// Generates the JSON search index as a serialized string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn generate_search_index() -> Result<String> {
    let entries = build_search_index();
    let json = serde_json::to_string(&entries)
        .map_err(|e| anyhow::anyhow!("Failed to serialize search index: {}", e))?;
    Ok(json)
}

/// Returns the lightweight client-side search JavaScript.
pub fn search_js() -> &'static str {
    r#"// UOR Foundation — client-side search
// Reads search-index.json and filters results on input

(function () {
  'use strict';

  const input = document.getElementById('search-input');
  const results = document.getElementById('search-results');

  if (!input || !results) return;

  let index = [];

  fetch('/search-index.json')
    .then(function (r) { return r.json(); })
    .then(function (data) { index = data; })
    .catch(function (e) { console.error('Failed to load search index:', e); });

  input.addEventListener('input', function () {
    const query = input.value.trim().toLowerCase();
    results.innerHTML = '';

    if (query.length < 2) return;

    const matches = index.filter(function (entry) {
      return entry.label.toLowerCase().includes(query) ||
             entry.description.toLowerCase().includes(query);
    }).slice(0, 20);

    if (matches.length === 0) {
      const li = document.createElement('li');
      li.textContent = 'No results.';
      results.appendChild(li);
      return;
    }

    matches.forEach(function (entry) {
      const li = document.createElement('li');
      const a = document.createElement('a');
      a.href = entry.url;
      a.textContent = entry.label;
      const kind = document.createElement('span');
      kind.className = 'result-kind';
      kind.textContent = entry.kind;
      const desc = document.createElement('p');
      desc.className = 'result-desc';
      desc.textContent = entry.description;
      li.appendChild(a);
      li.appendChild(kind);
      li.appendChild(desc);
      results.appendChild(li);
    });
  });
}());
"#
}
