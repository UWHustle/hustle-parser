#include <string.h>

#include "stringify.h"
#include "string_builder.h"

#define INDENT_SIZE 2

void new_line(string_builder *builder, int indent) {
    append_fmt(builder, "\n");
    for (int i = 0; i < indent * INDENT_SIZE; i++) {
        append_fmt(builder, " ");
    }
}

void json_traverse(string_builder *builder, parse_node *node, int indent) {
    append_fmt(builder, "{");
    indent++;
    new_line(builder, indent);
    append_fmt(builder, "\"type\": \"%s\"", node->type);

    for (size_t i = 0; i < node->value_keys->size; i++) {
        append_fmt(builder, ",");
        new_line(builder, indent);
        char *name = node->value_keys->array[i];
        char *value = node->value_values->array[i];
        append_fmt(builder, "\"%s\": \"%s\"", name, value);
    }

    for (size_t i = 0; i < node->child_keys->size; i++) {
        append_fmt(builder, ",");
        new_line(builder, indent);
        char *name = node->child_keys->array[i];
        append_fmt(builder, "\"%s\": ", name);

        parse_node *child = node->child_values->array[i];
        json_traverse(builder, child, indent);
    }

    for (size_t i = 0; i < node->child_list_keys->size; i++) {
        append_fmt(builder, ",");
        new_line(builder, indent);
        char *name = node->child_list_keys->array[i];
        append_fmt(builder, "\"%s\": [", name);

        dynamic_array *child_list = node->child_list_values->array[i];
        for (size_t j = 0; j < child_list->size; j++) {
            new_line(builder, indent + 1);
            json_traverse(builder, child_list->array[j], indent + 1);
            if (j < child_list->size - 1) {
                append_fmt(builder, ",");
            }
        }
        new_line(builder, indent);
        append_fmt(builder, "]");
    }
    new_line(builder, indent - 1);
    append_fmt(builder, "}");
}

char *json_stringify(parse_node *node) {
    string_builder *builder = alloc_builder();
    json_traverse(builder, node, 0);
    char *output = to_string(builder);
    free_builder(builder);
    return output;
}
