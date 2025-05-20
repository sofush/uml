import init, { on_redraw } from '/static/wasm/uml_wasm.js';

function cb() {
	on_redraw();
	window.requestAnimationFrame(cb);
};

init().then(async () => {
	console.log('Webassembly has loaded.');
	window.requestAnimationFrame(cb);
});

document.addEventListener('DOMContentLoaded', () => {
	const dialogEls = document.getElementsByTagName('dialog');

	Array.from(dialogEls).forEach(dialog => {
		const closeEls = dialog.getElementsByClassName('close');

		Array.from(closeEls).forEach(close => {
			close.addEventListener('click', _ => {
				dialog.open = false;
			});
		});
	});

	const forms = document.getElementsByTagName('form');

	Array.from(forms).forEach(form => {
		const inputEls = form.getElementsByTagName('input');
		if (inputEls.length === 0) return;

		inputEls[0].addEventListener('input', _ => {
			if (inputEls[0].value === '') {
				inputEls[0].ariaInvalid = 'true';
			} else {
				inputEls[0].ariaInvalid = undefined;
			}
		});

		form.addEventListener('submit', e => {
			e.preventDefault();
		});
	});
});
