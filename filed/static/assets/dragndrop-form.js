(
    () => {
        // display the js only text
        const text = document.getElementById('drag-n-drop-jsonly');
        text.style.display = 'initial';

        /** @type {HTMLElement} */
        const root_drag_rop = document.getElementsByClassName('file-drag-n-drop')[0];

        // make the root drag&drop element an ideal circle

        function updateDragNDrop() {
            
            if (document.body.scrollWidth < 667) {
                // mobile
                delete root_drag_rop.style.width;
                delete root_drag_rop.style.height;
                return
            }

            const width = root_drag_rop.offsetWidth;

            root_drag_rop.style.width   = width + 'px';
            root_drag_rop.style.height  = width + 'px';
        }

        updateDragNDrop();
        document.onresize = updateDragNDrop();

        /** @type {HTMLElement} */
        const drag_rop = document.getElementsByClassName('file-drag-n-drop-inside')[0];
        const dr_rop_t = document.getElementsByClassName('file-drag-n-drop-inside-text')[0];

        /** @type {HTMLInputElement} */
        const input_el = document.getElementById('bfile-formupload-file');

        /** @param {File} file */
        async function selectFile(file) {
            text.innerText = 'Processing...';
            drdrop_enabled = false;
            await fill_bg();
            drdrop_enabled = true;
            text.innerText = `Selected file: ${file.name.substring(0,16)}${file.name.length > 16 ? '...' : ''}\nYou can drop another file to replace this one`;

            const transfer = new DataTransfer();
            transfer.items.add(file);
            input_el.files = transfer.files;
        }

        root_drag_rop.onclick = e => {
            input_el.click();
        }

        input_el.onchange = e => {
            const file = input_el.files[0]
            selectFile(file);
        }

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

            if (e.dataTransfer.items.length != 0) {
                let file = e.dataTransfer.items[0].getAsFile();
                selectFile(file);
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