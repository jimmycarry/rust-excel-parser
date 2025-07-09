#!/usr/bin/env python3
"""
Create a test Excel file for testing the parser
"""
import pandas as pd
import os

def create_test_excel():
    """Create a test Excel file with multiple sheets"""
    
    # Create test data
    sheet1_data = {
        'Name': ['John Doe', 'Jane Smith', 'Bob Johnson'],
        'Age': [25, 30, 35],
        'City': ['New York', 'Los Angeles', 'Chicago'],
        'Salary': [50000, 60000, 70000]
    }
    
    sheet2_data = {
        'Product': ['Laptop', 'Mouse', 'Keyboard'],
        'Price': [999.99, 25.50, 75.00],
        'Stock': [10, 50, 25]
    }
    
    sheet3_data = {
        'Date': ['2024-01-01', '2024-01-02', '2024-01-03'],
        'Sales': [1000, 1200, 800],
        'Profit': [200, 300, 150]
    }
    
    # Create DataFrames
    df1 = pd.DataFrame(sheet1_data)
    df2 = pd.DataFrame(sheet2_data)
    df3 = pd.DataFrame(sheet3_data)
    
    # Write to Excel file
    with pd.ExcelWriter('test_data.xlsx', engine='openpyxl') as writer:
        df1.to_excel(writer, sheet_name='Employees', index=False)
        df2.to_excel(writer, sheet_name='Products', index=False)
        df3.to_excel(writer, sheet_name='Sales', index=False)
    
    print("Created test_data.xlsx with 3 sheets:")
    print("- Employees: 3 rows, 4 columns")
    print("- Products: 3 rows, 3 columns")
    print("- Sales: 3 rows, 3 columns")

if __name__ == "__main__":
    create_test_excel()