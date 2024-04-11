mod my_module;

use my_module::nested_module::my_nested_function;
use my_module::another_module;
use my_module::my_function;

fn main()
{
    my_function();
    my_nested_function();
    another_module::my_another_function();
    another_module::demo_fn();
}