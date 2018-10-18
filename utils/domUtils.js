export const appendStringToBody = (value) => {
    const text = document.createTextNode(value);
    document.body.appendChild(text);
};

export const alert = (message) => window.alert(message);
