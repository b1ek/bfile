(
    () => {
        // display the js only text
        const text = document.getElementById('drag-n-drop-jsonly');
        text.style.display = 'initial';

        /** @type {HTMLElement} */
        const root_drag_rop = document.getElementsByClassName('file-drag-n-drop')[0];
        /** @type {HTMLElement} */
        const drag_rop = document.getElementsByClassName('file-drag-n-drop-inside')[0];
        const dr_rop_t = document.getElementsByClassName('file-drag-n-drop-inside-text')[0];

        function drag_end() {
            root_drag_rop.style.background = 'var(--header-sec-color)'
            drag_rop.style.borderColor = '#656565';
        }

        async function fill_bg() {
            for (let i = 100; i != 0; i--) {
                await delay(Math.random());
                root_drag_rop.style.background = `linear-gradient(#353535 ${i}%, var(--header-sec-color) ${i + 0.01}%)`;
            }
            root_drag_rop.style.background = `var(--header-sec-color)`;
        }

        let drdrop_enabled = true;

        drag_rop.addEventListener('drop', async e => {
            e.preventDefault();

            if (!drdrop_enabled) return;

            if (e.dataTransfer.items) {
                for (let i = 0; i != e.dataTransfer.items.length; i++) {
                    let file = e.dataTransfer.items[i].getAsFile();
                    text.innerText = 'Processing...';
                    drdrop_enabled = false;
                    await fill_bg();
                    drdrop_enabled = true;
                    text.innerText = `Selected file: ${file.name.substring(0,16)}${file.name.length > 16 ? '...' : ''}\nYou can drop another file to replace this one`;
                }
            }
        });
        drag_rop.addEventListener('dragover', e => {
            e.preventDefault();
            if (!drdrop_enabled) return;

            root_drag_rop.style.background = '#393939'
            drag_rop.style.borderColor = '#959595';
        });
        
        drag_rop.addEventListener('mouseleave', drag_end);
        drag_rop.addEventListener('dragend',    drag_end)
    }
)()