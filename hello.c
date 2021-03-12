#include <sys/param.h>
#include <sys/module.h>
#include <sys/kernel.h>
#include <sys/systm.h>

extern int module_event(struct module *, int, void *);

static moduledata_t module_data = {
    "hello",      /* module name */
    module_event, /* event handler */
    NULL          /* extra data */
};

// Register the module with the kernel using:
//  the module name
//  our recently defined moduledata_t struct with module info
//  a module type (we're daying it's a driver this time)
//  a preference as to when to load the module
DECLARE_MODULE(hello, module_data, SI_SUB_DRIVERS, SI_ORDER_MIDDLE);