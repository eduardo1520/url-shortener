const BASE = 'http://localhost:3000';
const entries = [];

async function shorten() {
  const url = document.getElementById('url-input').value.trim();
  const err = document.getElementById('error-msg');
  err.classList.add('hidden');

  if (!url) return;

  try {
    const res = await fetch(`${BASE}/api/shorten`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url })
    });
    const data = await res.json();

    if (!res.ok) {
      err.textContent = data.error || 'Erro ao encurtar URL';
      err.classList.remove('hidden');
      return;
    }

    document.getElementById('result-code').textContent = data.code;
    document.getElementById('result-short').textContent = `${BASE}/r/${data.code}`;
    document.getElementById('result-box').classList.remove('hidden');

    entries.unshift({ code: data.code, url });
    renderHistory();
    document.getElementById('url-input').value = '';

  } catch (e) {
    err.textContent = 'Não foi possível conectar em localhost:3000. O servidor está rodando?';
    err.classList.remove('hidden');
  }
}

function copyCode() {
  const short = document.getElementById('result-short').textContent;
  navigator.clipboard.writeText(short);

  const btn = document.getElementById('copy-btn');
  btn.innerHTML = '<i class="ti ti-check text-sm"></i> copiado!';
  btn.classList.add('text-green-400', 'border-green-800');

  setTimeout(() => {
    btn.innerHTML = '<i class="ti ti-copy text-sm"></i> copiar link';
    btn.classList.remove('text-green-400', 'border-green-800');
  }, 2000);
}

function renderHistory() {
  document.getElementById('history').classList.remove('hidden');
  document.getElementById('history-list').innerHTML = entries.map(e => `
    <div class="flex items-center gap-3 py-2.5 border-b border-zinc-800 last:border-0 text-sm">
      <span class="bg-zinc-800 text-zinc-300 rounded-md px-2 py-0.5 text-xs font-medium tracking-wider shrink-0">${e.code}</span>
      <span class="text-zinc-500 truncate flex-1">${e.url}</span>
      <a href="${BASE}/r/${e.code}" target="_blank" class="text-zinc-600 hover:text-zinc-300 transition-colors shrink-0">
        <i class="ti ti-external-link text-sm"></i>
      </a>
    </div>
  `).join('');
}

document.getElementById('url-input').addEventListener('keydown', e => {
  if (e.key === 'Enter') shorten();
});