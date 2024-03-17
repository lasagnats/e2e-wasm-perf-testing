using DataStore.DB;
using Microsoft.OpenApi.Models;

var builder = WebApplication.CreateBuilder(args);
var dataBuilder = new DataBuilder();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc(
        "v1",
        new OpenApiInfo
        {
            Title = "DataStore API",
            Description = "Random Data generator APIs",
            Version = "v1"
        }
    );
});

builder.Services.AddCors(options =>
{
    options.AddDefaultPolicy(builder =>
    builder.WithOrigins("*")
           .AllowAnyMethod()
           .AllowAnyHeader());
});

var app = builder.Build();

app.UseSwagger();
app.UseSwaggerUI(c =>
{
    c.SwaggerEndpoint("/swagger/v1/swagger.json", "DataStore API V1");
});

app.MapGet("/data/{count}", (int count) => dataBuilder.BuildData(count));

app.UseCors();

app.Run();
