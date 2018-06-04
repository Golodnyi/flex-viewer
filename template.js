function onClickSelect(index) {
    value = Number(document.getElementsByClassName("select_" + index)[0].value);

    Array.from(document.getElementById("table").rows).forEach(function (element, rowIndex) {
        if (!Number(rowIndex)) {
            return;
        }

        colValue = Number(element.cells[index].innerHTML);
        switch (value) {
            case 1:
                if (colValue < 500) {
                    element.cells[index].innerHTML = 500;
                } else {
                    element.cells[index].innerHTML = Math.round(colValue / 1000 - 0.5);
                }
                break;
            case 2:
                element.cells[index].innerHTML = Math.round((0.2 * colValue - 2.8) / 1000);
                break
            case 3:
                if (colValue < 100) {
                    element.cells[index].innerHTML = 0;
                } else {
                    element.cells[index].innerHTML = Math.round(-0.6808 * ((colValue / 1000) * 2) - 1.4032 * colValue / 1000 + 11.902);
                }
                break;
            case 4:
                element.cells[index].innerHTML = colValue / 500;
                break;
        }
    });
}

function renderSelects() {
    Array.from(document.getElementById("table").rows[0].cells).forEach(function (element, index) {
        element.innerHTML = `<select class="select_` + index + `" onchange="onClickSelect(` + index + `)">
                                    <option value="0">None</option>
                                    <option value="1">Давление воздуха</option>
                                    <option value="2">Вакуум</option>
                                    <option value="3">Давление масла (грибок)</option>
                                    <option value="4">Давление масла (китайский)</option>
                                </select>` + element.innerHTML;
    });
}
renderSelects();