function onClickSelect(index) {
    value = Number(document.getElementsByClassName("select_" + index)[0].value);

    Array.from(document.getElementById("table").rows).forEach(function (element, rowIndex) {
        if (!Number(rowIndex)) {
            return;
        }

        colValue = Number(element.cells[index].getAttribute("value"));
        switch (value) {
            case 0:
                element.cells[index].innerHTML = colValue;
                break;
            case 1:
                var date = new Date(colValue*1000);
                var day = "0" + date.getDate();
                var month = "0" + (date.getMonth() + 1);
                var year = date.getFullYear();
                var hours = "0" + date.getHours();
                var minutes = "0" + date.getMinutes();
                var seconds = "0" + date.getSeconds();

                var formattedTime = day.substr(-2) + '.' + month.substr(-2) + '.' + year + ' ' + hours.substr(-2) + ':' + minutes.substr(-2) + ':' + seconds.substr(-2);
                element.cells[index].innerHTML = formattedTime;
                break;
            case 2:
                if (colValue < 500) {
                    element.cells[index].innerHTML = 500;
                } else {
                    element.cells[index].innerHTML = Math.round(colValue / 1000 - 0.5);
                }
                break;
            case 3:
                element.cells[index].innerHTML = Math.round((0.2 * colValue - 2.8) / 1000);
                break
            case 4:
                if (colValue < 100) {
                    element.cells[index].innerHTML = 0;
                } else {
                    element.cells[index].innerHTML = Math.round(-0.6808 * ((colValue / 1000) * 2) - 1.4032 * colValue / 1000 + 11.902);
                }
                break;
            case 5:
                element.cells[index].innerHTML = colValue / 500;
                break;
        }
    });
}

function renderSelects() {
    Array.from(document.getElementById("table").rows[0].cells).forEach(function (element, index) {
        element.innerHTML = element.innerHTML + `<hr/><select style="width: 44px;" class="select_` + index + `" onchange="onClickSelect(` + index + `)">
                                    <option value="0">Микровольты</option>
                                    <option value="1">Время</option>
                                    <option value="2">Давление воздуха</option>
                                    <option value="3">Вакуум</option>
                                    <option value="4">Давление масла (грибок)</option>
                                    <option value="5">Давление масла (китайский)</option>
                                </select>`;
    });
}
renderSelects();
Array.from(document.getElementById("table").rows[0].cells).forEach(function (element, index) {
    if (element.innerHTML.substr(0, 4) == "Time") {
        document.getElementsByClassName("select_" + index)[0].value = 1;
    } else if (element.innerHTML.substr(0, 11) == "StateU_Ain1" || element.innerHTML.substr(0, 11) == "StateU_Ain2") {
        document.getElementsByClassName("select_" + index)[0].value = 2;
    } else if (element.innerHTML.substr(0, 11) == "StateU_Ain3" || element.innerHTML.substr(0, 11) == "StateU_Ain4") {
         document.getElementsByClassName("select_" + index)[0].value = 3;
    } else if (element.innerHTML.substr(0, 11) == "StateU_Ain5") {
        document.getElementsByClassName("select_" + index)[0].value = 4;
    }
    onClickSelect(index);
});