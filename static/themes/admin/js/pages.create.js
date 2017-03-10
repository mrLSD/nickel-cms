jQuery(function($) {
    jQuery(window).on('load',function() {
        (function($) {
            jQuery('.area').redactor({'convertDivs':false,'imageUpload':'/admin/pages/uploadUserImage.html'});
            $(".tabset").jtabs();
            $(document.body).on("click",".show-additional", function(){
                $(this).toggleClass("open").parents("form").find(".additional-options").slideToggle();
                return false;
            });
        })(jQuery);
    });
});

