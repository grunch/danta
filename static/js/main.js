const App = {
  endpoint: "/api",
  interval: null,
  server: "https://lightninghackday.info",
};

App.init = () => {
  $(".attendee-form").collapse("show");
  $("#form").on("submit", App.submit);
};

App.submit = async (e) => {
  try {
    e.preventDefault();
    const firstname = $("#firstname").val();
    const data1 = $("#data1").val();
    const email = $("#email").val();

    const response = await App.makeRequest({
      api: "invoice",
      post: { firstname, data1, email },
    });

    if (!response) console.error("Error getting data!");
    if (response.success) {
      $(".attendee-form").collapse("hide");
      $("#invoice").collapse("show");

      $("#invoice-text").text(response.request);
      $("#invoice-text").attr("data-clipboard-text", response.request);

      const clipboard = new ClipboardJS("#invoice-text");
      tooltip = new bootstrap.Tooltip($("#invoice-text"), {
        trigger: "click",
        title: "Factura copiada",
        delay: 600,
      });
      clipboard.on("success", function (e) {
        tooltip.show();
      });

      $("#open-wallet").attr("href", `lightning:${response.request}`);
      $("#invoice-memo").text(response.description);
      $("#invoice-amount").text(`${response.amount} `);
      const qrCode = App.qrCode(response.request.toUpperCase(), 400);
      $("#qr-code").html(qrCode);
      $("html, body").animate(
        {
          scrollTop: $("#ancla").offset().top,
        },
        1000
      );
      App.interval = setInterval(App.waitPayment, 1000, response.hash);
    }
  } catch (error) {
    console.log(error.responseJSON);
  }
};

App.waitPayment = async (hash) => {
  const response = await App.makeRequest({
    api: `invoice/${hash}`,
  });
  if (response.paid) {
    clearInterval(App.interval);
    App.interval = null;
    $("#invoice").collapse("hide");
    const url = `${App.server}/verify/${response.preimage}`;
    const qrCode = App.qrCode(url, 400);
    $("#ticket-qr-code").html(qrCode);
    $("#pdffile").attr("href", `/files/${response.preimage}.pdf`);
    $("#success-box").collapse("show");
  }
};

/** Get qr code
  {
    text: <String>
  }

  @returns
  <QR Code Img Object>
*/
App.qrCode = (text) => {
  const back = "rgb(250, 250, 250)";
  const rounded = 100;
  const size = 300;

  const qr = kjua({ back, rounded, size, text });

  $(qr).css({ height: "auto", "max-width": "200px", width: "100%" });

  return qr;
};

App.makeRequest = ({ api, post }) => {
  const type = !post ? "GET" : "POST";
  const data = !!post ? JSON.stringify(post) : null;
  return $.ajax(`${App.endpoint}/${api}`, {
    type,
    data,
    contentType: "application/json",
    dataType: "json",
  });
};

$(() => App.init());
