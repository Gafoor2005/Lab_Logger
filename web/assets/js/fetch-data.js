let tbodyElement =  document.querySelector('tbody');
let totalCountElem =  document.getElementById('total-count');
let switchDbBtn =  document.querySelector("#switch-db p");
let switchDbDropdown =  document.getElementById("switch-db-dropdown");

function generateMasterRecord(s_id,lab,branch, yearSem, session, datetime) {
    return `
        <tr class=" text-gray-700 dark:text-gray-400 dark:hover:bg-gray-900 hover:bg-blue-100 hover:cursor-pointer ">
            <td class="px-4 py-3">
            <!-- <div class="flex items-center text-sm"> -->
                <!-- Avatar with inset shadow -->
                
                <div>
                <p class="font-semibold capitalize">${lab}</p>
                <p class="text-xs text-gray-300 dark:text-gray-600 s_id">
                    ${s_id}
                </p>
                </div>
            <!-- </div> -->
            </td>
            <td class="px-4 py-3 text-sm font-semibold">
            ${branch}
            </td>
            <td class="px-4 py-3 text-sm font-semibold">
            ${yearSem}
            </td>
            <td class="px-4 py-3 text-sm font-semibold">
            ${session}
            </td>
            <td class="px-4 py-3 text-sm">
            ${datetime}
            </td>
        </tr>
    `
}
function generateDbDropdownOption(display,id) {
    return `
        <div class="py-1" role="none">
            <a href="?db=${id.split(".")[0]}" class="block px-4 py-2 text-sm text-gray-300" role="menuitem" tabindex="-1" id="${id.split(".")[0]}">${display}</a>
        </div>
    `
}


function addQueryParam(paramName, paramValue) {
    const url = new URL(window.location.href);
    if (!url.searchParams.has(paramName)) {
        url.searchParams.set(paramName, paramValue);
        window.history.pushState({}, '', url.href);
    }
}
function getQueryParam(paramName) {
    const url = new URL(window.location.href);
    return url.searchParams.get(paramName);
}




// fetching avaliable db's
async function getDbList() {
    try {
        const response = await fetch("/list_db");
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const json = await response.json();
        return json;
    } catch (error) {
        console.error(error.message);
    }
}

// fetching master
async function getMaster(db) {
    try {
        const response = await fetch(`/db/${db}`);
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const json = await response.json();
        return json;
    } catch (error) {
        console.error(error.message);
    }
}


async function showDbList() {
    let db_list = await getDbList();
    // console.log(db_list);
    // console.log("dbs : " + db_list.files.length);
    if (db_list.files.length == 0){
        // console.log("ee");
        switchDbBtn.innerHTML = "data is empty";
        totalCountElem.innerText = "0";
    }
    const url = new URL(window.location.href);
    let activeDb = getQueryParam("db");
    
    if(!url.searchParams.has("db")){
        activeDb = db_list.files.reduce((prev, current) => {
            const prevMonth = parseInt(prev.split('-')[0]);
            const prevYear = parseInt(prev.split('-')[1].split('.')[0]);
            const currentMonth = parseInt(current.split('-')[0]);
            const currentYear = parseInt(current.split('-')[1].split('.')[0]);

            if (prevYear > currentYear) return prev;
            if (prevYear === currentYear && prevMonth > currentMonth) return prev;
            return current;
        }, db_list.files[0]).split(".")[0];
        addQueryParam("db",activeDb);
    }
    // console.log(activeDb);

    showMasterRecords(activeDb);
    let ind = db_list.files.indexOf(activeDb+".db");
    
    switchDbBtn.innerHTML = db_list.formatted_files[ind];
    for (let i = 0; i < db_list.files.length; i++) {
        switchDbDropdown.innerHTML += generateDbDropdownOption(db_list.formatted_files[i],db_list.files[i]);
    }
}
showDbList();

async function showMasterRecords(db) {
    // console.log(db);
    
    let masterRecords = await getMaster(db);
    // console.log(masterRecords);

    masterRecords.forEach(rec => {
        const date = new Date(rec.timestamp);

        const options = {
            year: 'numeric',
            month: 'short',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
            hour12: true,
        };
        tbodyElement.innerHTML += generateMasterRecord(rec.table_id.split('_')[1], rec.course, rec.branch, `${rec.year=='4'?'IV':'I'.repeat(rec.year)}-${'I'.repeat(rec.semester)}`, rec.session, date.toLocaleString('en-US', options))
    });
    totalCountElem.innerText = masterRecords.length;
    
    // print recs
    let recs = document.querySelectorAll("tbody > tr");
    recs.forEach(rec => {
        rec.addEventListener('click',()=>{
            let sIdElem = rec.querySelector('.s_id');
            printSessionData("s_"+sIdElem.innerText);
        })
    });

}

async function printSessionData(sId) {
    const url = new URL(window.location.href);
    let db = getQueryParam("db");
    let studentRecords = await getSessionRecords(db,sId);
    let masterRecords = await getMaster(db)
    // console.log(masterRecords);
    
    
    let masterData;
    masterRecords.forEach(e => {
        if(e.table_id == sId){
            masterData = e;
        }
    });
    // console.log(masterData);
    
    let config = await getConfig();
    printSession(config,masterData,studentRecords);

}

async function getSessionRecords(db,sId) {
    try {
        const response = await fetch(`/db/${db}/${sId}`);
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const json = await response.json();
        return json;
    } catch (error) {
        console.error(error.message);
    }
}
async function getConfig() {
    try {
        const response = await fetch("/config");
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }

        const json = await response.json();
        return json;
    } catch (error) {
        console.error(error.message);
    }
}

function capitalizeLab(lab){
    lab = lab.toUpperCase();
    let ind = lab.lastIndexOf('LAB');
    if(ind != -1){
       lab = lab.slice(0,ind) + "Lab";
    }
    return lab
}



let labRoomElem = document.getElementById('lab');
let dateElem = document.getElementById('date');
let sessionElem = document.getElementById('session');
let yearElem = document.getElementById('year');
let branchElem = document.getElementById('branch');
let labNameElem = document.getElementById('labName');
let contentElem = document.getElementById('content');
let headerElem = document.querySelector('.header');

function printSession(config,masterData,studentRecords){
    // change according to lab type
    labRoomElem.innerText = config.room_no + " " + capitalizeLab(config.lab_names[0]);
    sessionElem.innerText = masterData.session;
    yearElem.innerText = `${masterData.year=='4'?'IV':'I'.repeat(masterData.year)}-${'I'.repeat(masterData.semester)}`;
    branchElem.innerText = masterData.branch;
    labNameElem.innerText = masterData.course;
    const date = new Date(masterData.timestamp);
    const options = {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        // hour: '2-digit',
        // minute: '2-digit',
        // hour12: true,
    };
    dateElem.innerText = date.toLocaleString('en-US', options);
    // config.lab_type = "Single";

    let importElems = 
    `
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Lekton&display=swap" rel="stylesheet">
    `

    let styleElem = 
    `
        <style>
            #printable_area, .header, .data,.footer-wrapper{
                display: none;
            }
            body{
                display:flex;
            }
            #print_info{
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                margin: auto;
                width: fit-content;
            }
            @media print{
                #print_info{
                    display:none;
                }
                body, #printable_area, .header, .data,.footer-wrapper{
                    display: block;
                }

                *{
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                }
                .branding{
                    gap: 1em;
                    width: fit-content;
                    margin: 1em auto;
                    display: flex;
                    /* background-color: blueviolet; */
                }
                .branding > img, .branding > .empty {
                    width: 4em;
                    height: 4em;
                    border-radius: 100%;
                }
                .title{
                    /* TODO: check */
                    text-align: center;
                }
                .title > h1{
                    font-size: 1.2em;
                }
                .title > h4{
                    font-size: .8em;
                }
                .meta > h4{
                    text-align: center;
                }
                #printable_area{
                    width: 90%;
                    margin: auto;
                }

            

                .meta-table{
                    font-weight: 500;
                    color: grey;
                    font-size: 0.8em;
                    display: flex;
                    /* column-gap: 4em; */
                    justify-content: space-between;
                    row-gap: 1em;
                    flex-wrap: wrap;
                    margin: 1em auto;

                }
                .bold{
                    font-size: 1.2em;
                    color: black;
                    font-weight: 600;
                }
                .data > div h6 {
                    background: #e8e8e8;
                    text-align: center;
                    border-top: 1px solid;
                    border-right: 1px solid;
                    padding: 1em .876em;
                }
                .data > div p {
                    text-align: center;
                    border-top: 1px solid;
                    border-right: 1px solid;
                    padding: 0 .5em;
                    font-family: "Lekton", monospace;
                    font-weight: 400;
                    font-style: normal;
                    font-size: medium;
                }
                .data > div  {
                    border-left: 1px solid;
                    border-bottom: 1px solid;
                    /* border: 2px solid; */
                    font-size: 1.2em;
                    display: grid;
                    width: fit-content;
                    height: fit-content;
                    /* font-size: 1.2em;
                    display: grid;
                    width: fit-content;
                    column-gap: 1em;
                    row-gap: .3em;
                    height: fit-content; */
                }
                .data{
                    display: flex;
                    justify-content: space-between;
                }
                .lekton-regular {
                    font-family: "Lekton", monospace;
                    font-weight: 400;
                    font-style: normal;
                }

                .footer >div{
                    display: flex;
                    justify-content: space-between;
                    align-items: start;
                }
                .footer > div > div > div{
                    display: flex;
                    align-items: end;
                    gap: 10px;
                }
                .footer > div > div{
                    display: grid;
                    width: 47%;
                    column-gap: 1em;
                }
                .underline{
                    width: 100%;
                    border-bottom: 1px solid;
                    height: 2em;
                }
                .footer .bold{
                    font-size: .8em;
                }
                .height{
                    height: 60vh;
                }
                .header {
                    position: ${config.lab_type == "Double"?"absolute":"fixed"};
                    top: 0;
                    /* left: 0; */
                    width: 90%;
                    margin: 0 auto;
                    height: fit-content; /* Adjust the height */
                    border-bottom: 1px solid #ccc;
                
                }
                .header-2 {
                    display: ${config.lab_type == "Double"?"block":"none"};
                    position: absolute;
                    top: 100%;
                    page-break-before: always;
                }
                .footer-wrapper {
                    position: fixed;
                    /* background-color: red; */
                    bottom: 0;
                    width: 90%;
                    margin: 0 auto;
                    height: fit-content; /* Adjust the height */
                    display: flex;
                    margin-bottom: 2em;
                    /* border-top: 1px solid #ccc; */
                }
                .footer{
                    /* background-color: blue; */
                    width: 100%;
                    height: fit-content;
                }
                .data {
                    padding-top: 280px; /* Match the header height */
                    padding-bottom: 0; /* Match the footer height */
                }
                .data {
                    page-break-inside: avoid;
                }
                #second-table {
                    page-break-before: always;
                }

            }
        </style>
    
    `



    let doubleRecords;
    let chunkedRecs;
    if(config.lab_type == "Double"){
        let divRecs = divideArray(studentRecords);
        chunkedRecs = chunkArray( divRecs[0],20);
        doubleRecords = chunkArray( divRecs[1],20);
        // console.log(chunkedRecs);
        // console.log(doubleRecords);
        
        // result.push(arr.slice(0, ));
    }else{
        chunkedRecs = chunkArray(studentRecords,20); 
    }
    
    let content = "";
    for (let i = 0; i < chunkedRecs.length; i+=2) {
        content += 
        `
            <div class="data">
            ${generateTable(chunkedRecs[i],i*20)}
            ${generateTable(chunkedRecs[i+1],(i+1)*20)}
            </div>
        `
    }

    if(config.lab_type == "Double"){
        for (let i = 0; i < doubleRecords.length; i+=2) {
            content += 
            `
                <div class="data" id="second-table">
                ${generateTable(doubleRecords[i],i*20)}
                ${generateTable(doubleRecords[i+1],(i+1)*20)}
                </div>
            `
        }
        contentElem.innerHTML = content;

        labRoomElem.innerText = config.room_no + " " + capitalizeLab(config.lab_names[1]);
        
        document.querySelector(".header-2").innerHTML = headerElem.innerHTML;
        labRoomElem.innerText = config.room_no + " " + capitalizeLab(config.lab_names[0]);



        let printableArea = document.getElementById('printable_area').innerHTML;


        let windowObj = window.open("","","width:900,height:700;");
        let a = windowObj.document.write(
            `
                <head>
                    ${importElems}
                    ${styleElem}
                </head>
                <body>
                    <div id="print_info">
                        press <b>Ctrl + P</b> to print<br>
                        or <b>close</b> this window
                    </div>
                    
                    <div id="printable_area">
                    ${printableArea}
                    </div>
                </body>
            `
        );
        windowObj.document.close();
        windowObj.print();
        
        return;
    }

    contentElem.innerHTML = content;




    let printableArea = document.getElementById('printable_area').innerHTML;


    let windowObj = window.open("","","width:900,height:700;");
    let a = windowObj.document.write(
        `
            <head>
                ${importElems}
                ${styleElem}
            </head>
            <body>
                <div id="print_info">
                    press <b>Ctrl + P</b> to print<br>
                    or <b>close</b> this window
                </div>
                
                <div id="printable_area">
                ${printableArea}
                </div>
            </body>
        `
    );
    windowObj.document.close();
    windowObj.print();

}

function chunkArray(arr, chunkSize) {
    let result = [];
    for (let i = 0; i < arr.length; i += chunkSize) {
        result.push(arr.slice(i, i + chunkSize));
    }
    return result;
}
function divideArray(arr) {
    const middleIndex = Math.floor(arr.length / 2);
    const firstHalf = arr.slice(0, middleIndex + 1);
    const secondHalf = arr.slice(middleIndex + 1);
    return [firstHalf, secondHalf];
}


function generateTable(studentRecords,ind){
    if(studentRecords == undefined) return "";
    let content = `<div id="data-left">
          <h6 style="grid-column: 1;">Sno</h6>
          <h6 style="grid-column: 2;">Roll Number</h6>
          <h6 style="grid-column: 3;">Entry Time</h6>
          <h6 style="grid-column: 4;">Exit Time</h6>`

    for (let i = 0; i < studentRecords.length; i++) {
        
        content += `
            <p>${ind+(i+1)}</p>
            <p>${studentRecords[i].roll_number}</p>
            <p>${studentRecords[i].entry_timestamp.split(" ")[1]}</p>
            <p>${studentRecords[i].exit_timestamp != null ? studentRecords[i].exit_timestamp.split(" ")[1]:"-"}</p>
        `
    }
    content += "</div>"
    return content;
}